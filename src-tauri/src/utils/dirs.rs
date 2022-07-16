use std::env::temp_dir;
use std::path::PathBuf;
use tauri::{
  api::path::{home_dir, resource_dir},
  Env, PackageInfo,
};

#[cfg(not(feature = "verge-dev"))]
static APP_DIR: &str = "clash-verge";
#[cfg(feature = "verge-dev")]
static APP_DIR: &str = "clash-verge-dev";

static CLASH_CONFIG: &str = "config.yaml";
static VERGE_CONFIG: &str = "verge.yaml";
static PROFILE_YAML: &str = "profiles.yaml";
static PROFILE_TEMP: &str = "clash-verge-runtime.yaml";

#[cfg(windows)]
static mut RESOURCE_DIR: Option<PathBuf> = None;

/// portable flag
#[allow(unused)]
static mut PORTABLE_FLAG: bool = false;

/// initialize portable flag
#[allow(unused)]
pub unsafe fn init_portable_flag() {
  #[cfg(target_os = "windows")]
  {
    use tauri::utils::platform::current_exe;

    let exe = current_exe().unwrap();
    let dir = exe.parent().unwrap();
    let dir = PathBuf::from(dir).join(".config/PORTABLE");

    if dir.exists() {
      PORTABLE_FLAG = true;
    }
  }
}

/// get the verge app home dir
pub fn app_home_dir() -> PathBuf {
  #[cfg(target_os = "windows")]
  unsafe {
    use tauri::utils::platform::current_exe;

    if !PORTABLE_FLAG {
      home_dir().unwrap().join(".config").join(APP_DIR)
    } else {
      let app_exe = current_exe().unwrap();
      let app_exe = dunce::canonicalize(app_exe).unwrap();
      let app_dir = app_exe.parent().unwrap();
      PathBuf::from(app_dir).join(".config").join(APP_DIR)
    }
  }

  #[cfg(not(target_os = "windows"))]
  home_dir().unwrap().join(".config").join(APP_DIR)
}

/// get the resources dir
pub fn app_resources_dir(package_info: &PackageInfo) -> PathBuf {
  let res_dir = resource_dir(package_info, &Env::default())
    .unwrap()
    .join("resources");

  #[cfg(windows)]
  unsafe {
    RESOURCE_DIR = Some(res_dir.clone());
  }

  res_dir
}

/// profiles dir
pub fn app_profiles_dir() -> PathBuf {
  app_home_dir().join("profiles")
}

/// logs dir
pub fn app_logs_dir() -> PathBuf {
  app_home_dir().join("logs")
}

pub fn clash_path() -> PathBuf {
  app_home_dir().join(CLASH_CONFIG)
}

pub fn verge_path() -> PathBuf {
  app_home_dir().join(VERGE_CONFIG)
}

pub fn profiles_path() -> PathBuf {
  app_home_dir().join(PROFILE_YAML)
}

pub fn profiles_temp_path() -> PathBuf {
  #[cfg(not(feature = "debug-yml"))]
  return temp_dir().join(PROFILE_TEMP);

  #[cfg(feature = "debug-yml")]
  return app_home_dir().join(PROFILE_TEMP);
}

#[cfg(windows)]
static SERVICE_PATH: &str = "clash-verge-service.exe";

#[cfg(windows)]
pub fn service_path() -> PathBuf {
  unsafe {
    let res_dir = RESOURCE_DIR.clone().unwrap();
    res_dir.join(SERVICE_PATH)
  }
}

#[cfg(windows)]
pub fn service_log_file() -> PathBuf {
  use chrono::Local;

  let log_dir = app_logs_dir().join("service");

  let local_time = Local::now().format("%Y-%m-%d-%H%M%S").to_string();
  let log_file = format!("{}.log", local_time);
  let log_file = log_dir.join(log_file);

  std::fs::create_dir_all(&log_dir).unwrap();

  log_file
}
