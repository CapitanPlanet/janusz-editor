<script setup lang="ts">
import { ref } from 'vue'
import { save, open } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'

const title = ref('Edytor Janusza V1.1')
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

function usunDialog(id: number) {
  dialogs.value = dialogs.value.filter(d => d.id !== id)
}

async function zapiszProjekt() {
  try {
    const path = await save({
      defaultPath: 'moja-gra.janusz',
      filters: [
        { name: 'Janusz Project', extensions: ['janusz'] },
        { name: 'JSON', extensions: ['json'] },
        { name: 'Wszystkie pliki', extensions: ['*'] }
      ]
    })
    if (path) {
      let finalPath = path
      if (!path.endsWith('.janusz') && !path.endsWith('.json')) {
        finalPath = path + '.janusz'
      }
      await writeTextFile(finalPath, JSON.stringify(dialogs.value, null, 2))
      console.log('Zapisano do:', finalPath)
      alert('Zapisane byku!')
    }
  } catch (e) {
    console.error('Błąd zapisu:', e)
    alert('Błąd zapisu: ' + e)
  }
}

async function wczytajProjekt() {
  try {
    const path = await open({
      multiple: false,
      filters: [
        { name: 'Janusz Project', extensions: ['janusz'] },
        { name: 'JSON', extensions: ['json'] },
        { name: 'Wszystkie pliki', extensions: ['*'] }
      ]
    })
    if (path) {
      console.log('Wczytuje z:', path)
      const content = await readTextFile(path as string)
      dialogs.value = JSON.parse(content)
      alert('Wczytano kult!')
    }
  } catch (e) {
    console.error('Błąd wczytywania:', e)
    alert('Błąd: ' + e)
  }
}
</script>

<template>
  <main class="container">
    <h1>{{ title }}</h1>
    
    <div class="toolbar">
      <button @click="dodajDialog">+ Dodaj dialog</button>
      <button @click="zapiszProjekt">💾 Zapisz Projekt</button>
      <button @click="wczytajProjekt">📂 Wczytaj Projekt</button>
    </div>

    <div v-for="d in dialogs" :key="d.id" class="dialog">
      <input v-model="d.speaker" class="speaker-input" placeholder="Postać" />
      <textarea v-model="d.text" class="text-input" placeholder="Tekst"></textarea>
      <button @click="usunDialog(d.id)" class="delete">🗑️</button>
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
.toolbar { margin: 1rem 0; }
button {
  background: #4ade80;
  color: #111;
  border: none;
  padding: 8px 16px;
  margin-right: 0.5rem;
  cursor: pointer;
  font-weight: bold;
}
button:hover { background: #22c55e; }
.dialog {
  border: 1px solid #333;
  padding: 1rem;
  margin-bottom: 0.5rem;
  background: #222;
  display: flex;
  gap: 8px;
  align-items: flex-start;
}
.speaker-input, .text-input {
  background: #111;
  border: 1px solid #444;
  color: #eee;
  padding: 4px;
}
.speaker-input { width: 120px; }
.text-input { flex: 1; min-height: 40px; }
.delete { background: #ef4444; }
.delete:hover { background: #dc2626; }
</style>