<script setup lang="ts">
import { ref } from 'vue'
import { open, save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs'

interface Choice {
  Text: string
  Next: string
  Cebula: number
  Wstyd: number
  Portfel: number
  Reputacja: number
  ReactionText: string
  ReactionImage: string
  SoundFile?: string
  FlagsSet?: string[]
  FlagsRequired?: string[]
  MinPortfel?: number | null
  KosztPortfel?: number | null
  FailText?: string
}

interface Scene {
  Id: string
  SceneTitle: string
  Background: string
  Text: string
  Choices: Choice[]
}

const scenes = ref<Scene[]>([
  {
    Id: "start",
    SceneTitle: "Start gry",
    Background: "images/bg_start.jpg",
    Text: "Tu wpisz tekst sceny...",
    Choices: [
      { Text: "Wybór 1", Next: "", Cebula: 0, Wstyd: 0, Portfel: 0, Reputacja: 0, ReactionText: "", ReactionImage: "" }
    ]
  }
])

const selectedSceneIndex = ref(0)

const addScene = () => {
  scenes.value.push({
    Id: `scena_${Date.now()}`,
    SceneTitle: "Nowa scena",
    Background: "",
    Text: "",
    Choices: []
  })
  selectedSceneIndex.value = scenes.value.length - 1
}

const deleteScene = (index: number) => {
  scenes.value.splice(index, 1)
  if (selectedSceneIndex.value >= scenes.value.length) {
    selectedSceneIndex.value = Math.max(0, scenes.value.length - 1)
  }
}

const addChoice = (sceneIndex: number) => {
  scenes.value[sceneIndex].Choices.push({
    Text: "Nowy wybór",
    Next: "",
    Cebula: 0,
    Wstyd: 0,
    Portfel: 0,
    Reputacja: 0,
    ReactionText: "",
    ReactionImage: ""
  })
}

const deleteChoice = (sceneIndex: number, choiceIndex: number) => {
  scenes.value[sceneIndex].Choices.splice(choiceIndex, 1)
}

const saveProject = async () => {
  try {
    const path = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })
    if (path) {
      await writeTextFile(path, JSON.stringify(scenes.value, null, 2))
      console.log('Zapisano do:', path)
    }
  } catch (error) {
    console.error('Błąd zapisu:', error)
  }
}

const loadProject = async () => {
  try {
    const selected = await open({
      filters: [{ name: 'JSON', extensions: ['json'] }]
    })
    if (selected) {
      const content = await readTextFile(selected as string)
      const parsed = JSON.parse(content)
      scenes.value = parsed
      selectedSceneIndex.value = 0
      console.log('Wczytano:', parsed.length, 'scen')
    }
  } catch (error) {
    console.error('Błąd wczytywania:', error)
    alert(`Błąd: ${error}`)
  }
}
</script>

<template>
  <main class="container">
    <h1>Edytor Janusza V1.2 - RPG Maker</h1>

    <div class="toolbar">
      <button @click="addScene">+ Dodaj scenę</button>
      <button @click="saveProject">💾 Zapisz Projekt</button>
      <button @click="loadProject">📂 Wczytaj Projekt</button>
    </div>

    <div class="editor-layout">
      <!-- LEWY PANEL: Lista scen -->
      <div class="scene-list">
        <h3>Sceny [{{ scenes.length }}]</h3>
        <div
          v-for="(scene, index) in scenes"
          :key="scene.Id"
          @click="selectedSceneIndex = index"
          :class="{ active: selectedSceneIndex === index }"
          class="scene-item"
        >
          <span>{{ scene.Id }}</span>
          <button @click.stop="deleteScene(index)" class="delete-btn">🗑️</button>
        </div>
      </div>

      <!-- PRAWY PANEL: Edycja sceny -->
      <div class="scene-editor" v-if="scenes[selectedSceneIndex]">
        <h3>Edycja: {{ scenes[selectedSceneIndex].Id }}</h3>

        <label>ID Sceny:</label>
        <input v-model="scenes[selectedSceneIndex].Id" placeholder="unikalne_id" />

        <label>Tytuł sceny:</label>
        <input v-model="scenes[selectedSceneIndex].SceneTitle" placeholder="DZIEŃ 1 - PONIEDZIAŁEK" />

        <label>Background:</label>
        <input v-model="scenes[selectedSceneIndex].Background" placeholder="images/bg_dom.jpg" />

        <label>Tekst sceny:</label>
        <textarea v-model="scenes[selectedSceneIndex].Text" rows="6" placeholder="6:47. Grażyna: 'Janusz...'" />

        <h4>Wybory [{{ scenes[selectedSceneIndex].Choices.length }}]</h4>
        <button @click="addChoice(selectedSceneIndex)">+ Dodaj wybór</button>

        <div v-for="(choice, cIndex) in scenes[selectedSceneIndex].Choices" :key="cIndex" class="choice-card">
          <div class="choice-header">
            <strong>Wybór {{ cIndex + 1 }}</strong>
            <button @click="deleteChoice(selectedSceneIndex, cIndex)" class="delete-btn">🗑️</button>
          </div>

          <label>Tekst wyboru:</label>
          <input v-model="choice.Text" />

          <label>Next ID:</label>
          <input v-model="choice.Next" placeholder="id_następnej_sceny" />

          <div class="stats-row">
            <div><label>Cebula:</label><input type="number" v-model.number="choice.Cebula" /></div>
            <div><label>Wstyd:</label><input type="number" v-model.number="choice.Wstyd" /></div>
            <div><label>Portfel:</label><input type="number" v-model.number="choice.Portfel" /></div>
            <div><label>Reputacja:</label><input type="number" v-model.number="choice.Reputacja" /></div>
          </div>

          <label>ReactionText:</label>
          <input v-model="choice.ReactionText" />

          <label>ReactionImage:</label>
          <input v-model="choice.ReactionImage" placeholder="images/janusz_smiech.png" />
        </div>
      </div>
    </div>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 14px;
  color: #0f0f0f;
  background-color: #f6f6f6;
}

.container {
  margin: 0;
  padding: 1rem;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

h1 { color: #2ecc71; margin: 0 0 1rem 0; }

.toolbar {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

button {
  border-radius: 6px;
  border: 1px solid transparent;
  padding: 0.5em 1em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  cursor: pointer;
}
button:hover { border-color: #2ecc71; }
button:active { background-color: #e8e8e8; }

.editor-layout {
  display: flex;
  gap: 1rem;
  flex: 1;
  overflow: hidden;
}

.scene-list {
  width: 250px;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  overflow-y: auto;
}

.scene-item {
  padding: 0.5rem;
  margin-bottom: 0.5rem;
  background: #f6f6f6;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.scene-item:hover { background: #e8e8e8; }
.scene-item.active { background: #2ecc71; color: white; }

.scene-editor {
  flex: 1;
  background: #fff;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  overflow-y: auto;
}

label {
  display: block;
  margin-top: 0.75rem;
  margin-bottom: 0.25rem;
  font-weight: 600;
  font-size: 0.9em;
}

input, textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: inherit;
  box-sizing: border-box;
}

.choice-card {
  background: #f9f9f9;
  border: 1px solid #ddd;
  border-radius: 6px;
  padding: 1rem;
  margin-top: 1rem;
}

.choice-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.stats-row {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 0.5rem;
  margin: 0.5rem 0;
}

.stats-row input { width: 100%; }

.delete-btn {
  background: #e74c3c;
  color: white;
  padding: 0.25rem 0.5rem;
  font-size: 0.9em;
}
.delete-btn:hover { background: #c0392b; }

@media (prefers-color-scheme: dark) {
  :root { color: #f6f6f6; background-color: #2f2f2f; }
 .scene-list,.scene-editor,.choice-card { background: #1a1a1a; border-color: #444; }
 .scene-item { background: #2f2f2f; }
 .scene-item:hover { background: #3f3f3f; }
  input, textarea { background: #2f2f2f; color: #f6f6f6; border-color: #444; }
  button { color: #ffffff; background-color: #0f0f0f; }
}
</style>