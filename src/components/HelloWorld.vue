<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
import { emit } from "@tauri-apps/api/event";
import { checkUpdate, installUpdate } from "@tauri-apps/api/updater";
import { listen } from "@tauri-apps/api/event";
try {
  emit("tauri://update");
  const { shouldUpdate, manifest } = await checkUpdate();

  if (shouldUpdate) {

    // display dialog
   await installUpdate();
    // install complete, ask to restart
  }
  listen("tauri://update-available", function (res) {
    console.log("New version available: ", res);
  });
 /* const { shouldUpdate, manifest } = await checkUpdate();

  if (shouldUpdate) {
    // display dialog
    await installUpdate();
    // install complete, ask to restart
  }*/
} catch(error) {
  console.log(error);
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter_b
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
