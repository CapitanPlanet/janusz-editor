<script setup lang="ts">
import { ref } from 'vue'

const title = ref('Edytor Janusza V1')
const dialogs = ref([
  { id: 1, speaker: 'Janusz', text: 'Witaj w edytorze, byku.' },
  { id: 2, speaker: 'Kultysta', text: 'Chcesz zagrać w grę?' }
])

function dodajDialog() {
  dialogs.value.push({
    id: Date.now(),
    speaker: 'Nowy',
    text: 'Nowy tekst żenady'
  })
}
</script>

<template>
  <main class="container">
    <h1>{{ title }}</h1>
    
    <button @click="dodajDialog">+ Dodaj dialog</button>

    <div v-for="d in dialogs" :key="d.id" class="dialog">
      <strong>{{ d.speaker }}:</strong> {{ d.text }}
    </div>
  </main>
</template>

<style>
.container {
  padding: 2rem;
  font-family: sans-serif;
  background: #1a1a1a;
  color: #eee;
  min-height: 100vh;
}
h1 { color: #4ade80; }
button {
  background: #4ade80;
  color: #111;
  border: none;
  padding: 8px 16px;
  margin: 1rem 0;
  cursor: pointer;
  font-weight: bold;
}
.dialog {
  border: 1px solid #333;
  padding: 1rem;
  margin-bottom: 0.5rem;
  background: #222;
}
</style>