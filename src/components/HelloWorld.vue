<script setup lang="ts">

import { invoke } from '@tauri-apps/api';
// import { error } from 'tauri-plugin-log-api'

async function openWindow() {
  await invoke('open_window');
}
</script>

<template>
  <div class="card">
    <button type="button" @click="openWindow()">New Window</button>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
