<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api';
import { error } from 'tauri-plugin-log-api'

defineProps<{
  msg: string,
}>()

const helloMessage = ref('')

async function hello() {
  const result: any = await invoke('hello', {
    name: 'World'
  });
  helloMessage.value = result.toString();
  error("hello", result)
}

const count = ref(0)
</script>

<template>
  <h1>{{ msg }}</h1>
  <h2 v-if="helloMessage != ''">{{ helloMessage }}</h2>

  <div class="card">
    <button type="button" @click="hello()">Hello</button>
    <button type="button" @click="count++">count is {{ count }}</button>
  </div>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
