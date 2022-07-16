use super::Core;
use crate::log_if_err;
use anyhow::{bail, Context, Result};
use delay_timer::prelude::{DelayTimer, DelayTimerBuilder, TaskBuilder};
use std::collections::HashMap;

type TaskID = u64;

pub struct Timer {
  /// cron manager
  delay_timer: DelayTimer,

  /// save the current state
  timer_map: HashMap<String, (TaskID, u64)>,

  /// increment id
  timer_count: TaskID,

  /// save the instance of the app
  core: Option<Core>,
}

impl Timer {
  pub fn new() -> Self {
    Timer {
      delay_timer: DelayTimerBuilder::default().build(),
      timer_map: HashMap::new(),
      timer_count: 1,
      core: None,
    }
  }

  pub fn set_core(&mut self, core: Core) {
    self.core = Some(core);
  }

  /// Correctly update all cron tasks
  pub fn refresh(&mut self) -> Result<()> {
    if self.core.is_none() {
      bail!("unhandle error for core is none");
    }

    let diff_map = self.gen_diff();

    for (uid, diff) in diff_map.into_iter() {
      match diff {
        DiffFlag::Del(tid) => {
          let _ = self.timer_map.remove(&uid);
          log_if_err!(self.delay_timer.remove_task(tid));
        }
        DiffFlag::Add(tid, val) => {
          let _ = self.timer_map.insert(uid.clone(), (tid, val));
          log_if_err!(self.add_task(uid, tid, val));
        }
        DiffFlag::Mod(tid, val) => {
          let _ = self.timer_map.insert(uid.clone(), (tid, val));
          log_if_err!(self.delay_timer.remove_task(tid));
          log_if_err!(self.add_task(uid, tid, val));
        }
      }
    }

    Ok(())
  }

  /// generate a uid -> update_interval map
  fn gen_map(&self) -> HashMap<String, u64> {
    let profiles = self.core.as_ref().unwrap().profiles.lock();

    let mut new_map = HashMap::new();

    if let Some(items) = profiles.get_items() {
      for item in items.iter() {
        if item.option.is_some() {
          let option = item.option.as_ref().unwrap();
          let interval = option.update_interval.unwrap_or(0);

          if interval > 0 {
            new_map.insert(item.uid.clone().unwrap(), interval);
          }
        }
      }
    }

    new_map
  }

  /// generate the diff map for refresh
  fn gen_diff(&mut self) -> HashMap<String, DiffFlag> {
    let mut diff_map = HashMap::new();

    let new_map = self.gen_map();
    let cur_map = &self.timer_map;

    cur_map.iter().for_each(|(uid, (tid, val))| {
      let new_val = new_map.get(uid).unwrap_or(&0);

      if *new_val == 0 {
        diff_map.insert(uid.clone(), DiffFlag::Del(*tid));
      } else if new_val != val {
        diff_map.insert(uid.clone(), DiffFlag::Mod(*tid, *new_val));
      }
    });

    let mut count = self.timer_count;

    new_map.iter().for_each(|(uid, val)| {
      if cur_map.get(uid).is_none() {
        diff_map.insert(uid.clone(), DiffFlag::Add(count, *val));

        count += 1;
      }
    });

    self.timer_count = count;

    diff_map
  }

  /// add a cron task
  fn add_task(&self, uid: String, tid: TaskID, minutes: u64) -> Result<()> {
    let core = self.core.clone().unwrap();

    let task = TaskBuilder::default()
      .set_task_id(tid)
      .set_maximum_parallel_runnable_num(1)
      .set_frequency_repeated_by_minutes(minutes)
      // .set_frequency_repeated_by_seconds(minutes) // for test
      .spawn_async_routine(move || Self::async_task(core.clone(), uid.clone()))
      .context("failed to create timer task")?;

    self
      .delay_timer
      .add_task(task)
      .context("failed to add timer task")?;

    Ok(())
  }

  /// the task runner
  async fn async_task(core: Core, uid: String) {
    log::info!("running timer task `{uid}`");
    log_if_err!(Core::update_profile_item(core, uid, None).await);
  }
}

#[derive(Debug)]
enum DiffFlag {
  Del(TaskID),
  Add(TaskID, u64),
  Mod(TaskID, u64),
}
