<script setup lang="ts">
import { ref, watch } from 'vue'
import { open, save } from '@tauri-apps/plugin-dialog'
import { readTextFile, writeTextFile, exists } from '@tauri-apps/plugin-fs'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { appDataDir, join } from '@tauri-apps/api/path'

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
    Background: "",
    Text: "6:47. Grażyna: 'Janusz wstawaj do roboty!'",
    Choices: [
      { Text: "Wstaję", Next: "kuchnia", Cebula: 0, Wstyd: 0, Portfel: 0, Reputacja: 1, ReactionText: "Wstałeś jak chłop", ReactionImage: "" }
    ]
  }
])

const selectedSceneIndex = ref(0)
const imageCache = ref<Record<string, string>>({})
const imageLoading = ref<Record<string, boolean>>({})

const resolveImagePath = async (path: string): Promise<string> => {
  if (!path) return ''
  if (path.startsWith('asset://')) return path

  if (path.startsWith('backgrounds/')) {
    try {
      const appDir = await appDataDir()
      const fullPath = await join(appDir, path)
      const fileExists = await exists(fullPath)
      console.log('[IMG] AppData:', fullPath, 'exists:', fileExists)
      if (!fileExists) return ''
      return convertFileSrc(fullPath)
    } catch (e) {
      console.error('[IMG] Błąd AppData:', e)
      return ''
    }
  }

  if (path.startsWith('images/')) {
    return path
  }

  if (path.match(/^[A-Z]:\\/i) || path.startsWith('/')) {
    try {
      return convertFileSrc(path)
    } catch (e) {
      console.error('[IMG] Błąd convertFileSrc:', e)
      return ''
    }
  }

  return path
}

const getImageSrc = (path: string) => {
  if (!path) return ''
  if (imageCache.value[path]) return imageCache.value[path]
  if (imageCache.value[path] === 'error') return ''
  if (imageLoading.value[path]) return ''

  imageLoading.value[path] = true
  resolveImagePath(path).then(resolved => {
    if (resolved) {
      imageCache.value[path] = resolved
    } else {
      imageCache.value[path] = 'error'
      console.error('[IMG] Nie udało się załadować:', path)
    }
    imageLoading.value[path] = false
  })

  return ''
}

watch(scenes, () => {
  const allPaths = new Set<string>()
  scenes.value.forEach(s => {
    if (s.Background) allPaths.add(s.Background)
    s.Choices.forEach(c => {
      if (c.ReactionImage) allPaths.add(c.ReactionImage)
    })
  })

  Object.keys(imageCache.value).forEach(key => {
    if (!allPaths.has(key)) delete imageCache.value[key]
  })
}, { deep: true })

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
  if (scenes.value.length <= 1) {
    alert('Nie możesz usunąć ostatniej sceny!')
    return
  }
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
      alert('Projekt zapisany!')
    }
  } catch (error) {
    console.error('Błąd zapisu:', error)
    alert(`Błąd zapisu: ${error}`)
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
      imageCache.value = {}
      alert(`Wczytano ${parsed.length} scen`)
    }
  } catch (error) {
    console.error('Błąd wczytywania:', error)
    alert(`Błąd: ${error}`)
  }
}

const pickBackground = async () => {
  console.log('[PICK] Start wyboru tła')
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] }]
  })

  if (!selected || typeof selected!== 'string') {
    console.log('[PICK] Anulowano wybór')
    return
  }

  console.log('[PICK] Wybrano:', selected)
  
  try {
    const newPath = await invoke<string>('copy_asset_file', { 
      srcPath: selected, 
      assetType: 'Backgrounds' 
    })
    
    scenes.value[selectedSceneIndex.value].Background = newPath
    delete imageCache.value[newPath]
    console.log('[PICK] SUKCES! Zapisano ścieżkę:', newPath)
    alert(`Skopiowano: ${newPath}`)
  } catch (e) {
    console.error('[PICK] BŁĄD KOPIOWANIA:', e)
    alert(`Błąd kopiowania!\n${e}`)
  }
}

const pickReactionImage = async (choiceIndex: number) => {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp', 'gif'] }]
  })

  if (!selected || typeof selected!== 'string') return

  try {
    const newPath = await invoke<string>('copy_asset_file', { 
      srcPath: selected, 
      assetType: 'Backgrounds' 
    })
    
    scenes.value[selectedSceneIndex.value].Choices[choiceIndex].ReactionImage = newPath
    delete imageCache.value[newPath]
  } catch (e) {
    console.error('[PICK] Błąd kopiowania:', e)
    alert(`Błąd kopiowania!\n${e}`)
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
          <button @click.stop="deleteScene(index)" class="delete-btn">🗑</button>
        </div>
      </div>

      <div class="scene-editor" v-if="scenes[selectedSceneIndex]">
        <h3>Edycja: {{ scenes[selectedSceneIndex].Id }}</h3>

        <div class="scene-preview">
          <img
            v-if="getImageSrc(scenes[selectedSceneIndex].Background)"
            :src="getImageSrc(scenes[selectedSceneIndex].Background)"
            alt="Podgląd tła"
          />
          <div v-else class="no-image">
            {{ scenes[selectedSceneIndex].Background? 'Ładowanie/błąd - sprawdź konsolę F12' : 'Brak tła - wybierz plik' }}
          </div>
        </div>

        <label>ID Sceny:</label>
        <input v-model="scenes[selectedSceneIndex].Id" placeholder="unikalne_id" />

        <label>Tytuł sceny:</label>
        <input v-model="scenes[selectedSceneIndex].SceneTitle" placeholder="DZIEŃ 1 - PONIEDZIAŁEK" />

        <label>Background:</label>
        <div style="display: flex; gap: 0.5rem;">
          <input
            v-model="scenes[selectedSceneIndex].Background"
            placeholder="Kliknij Wybierz ->"
            style="flex: 1;"
            readonly
          />
          <button @click="pickBackground" style="white-space: nowrap;">📁 Wybierz</button>
        </div>

        <label>Tekst sceny:</label>
        <textarea v-model="scenes[selectedSceneIndex].Text" rows="6" placeholder="6:47. Grażyna: 'Janusz...'" />

        <h4>Wybory [{{ scenes[selectedSceneIndex].Choices.length }}]</h4>
        <button @click="addChoice(selectedSceneIndex)">+ Dodaj wybór</button>

        <div v-for="(choice, cIndex) in scenes[selectedSceneIndex].Choices" :key="cIndex" class="choice-card">
          <div class="choice-header">
            <strong>Wybór {{ cIndex + 1 }}</strong>
            <button @click="deleteChoice(selectedSceneIndex, cIndex)" class="delete-btn">🗑</button>
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
          <div style="display: flex; gap: 0.5rem; align-items: center;">
            <input v-model="choice.ReactionImage" placeholder="Kliknij Wybierz ->" style="flex: 1;" readonly />
            <button @click="pickReactionImage(cIndex)" style="white-space: nowrap;">📁</button>
          </div>
          <div v-if="choice.ReactionImage && getImageSrc(choice.ReactionImage)" class="reaction-preview">
            <img :src="getImageSrc(choice.ReactionImage)" alt="Reaction" />
          </div>
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
  background-color: #1a1a1a;
}

body {
  margin: 0;
  position: relative;
  min-height: 100vh;
}

body::before {
  content: '';
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #1e3c72 0%, #2a5298 50%, #1e3c72 100%);
  opacity: 0.9;
  z-index: -1;
  filter: blur(2px);
}

.container {
  margin: 0;
  padding: 1rem;
  height: 100vh;
  display: flex;
  flex-direction: column;
  backdrop-filter: blur(10px);
}

h1 {
  color: #2ecc71;
  margin: 0 0 1rem 0;
  text-shadow: 0 2px 8px rgba(0,0,0,0.5);
}

.toolbar {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

button {
  border-radius: 6px;
  border: 1px solid rgba(255,255,255,0.1);
  padding: 0.5em 1em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #fff;
  background-color: rgba(255,255,255,0.1);
  cursor: pointer;
  backdrop-filter: blur(5px);
  transition: all 0.2s;
}
button:hover {
  border-color: #2ecc71;
  background-color: rgba(46, 204, 113, 0.2);
}
button:active { background-color: rgba(46, 204, 113, 0.3); }

.editor-layout {
  display: flex;
  gap: 1rem;
  flex: 1;
  overflow: hidden;
}

.scene-list {
  width: 250px;
  background: rgba(26, 26, 26, 0.85);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 8px;
  padding: 1rem;
  overflow-y: auto;
  backdrop-filter: blur(10px);
}

.scene-item {
  padding: 0.5rem;
  margin-bottom: 0.5rem;
  background: rgba(255,255,255,0.05);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: #fff;
  transition: all 0.2s;
}
.scene-item:hover { background: rgba(255,255,255,0.1); }
.scene-item.active { background: #2ecc71; color: #000; }

.scene-editor {
  flex: 1;
  background: rgba(26, 26, 26, 0.85);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 8px;
  padding: 1rem;
  overflow-y: auto;
  backdrop-filter: blur(10px);
  color: #fff;
}

label {
  display: block;
  margin-top: 0.75rem;
  margin-bottom: 0.25rem;
  font-weight: 600;
  font-size: 0.9em;
  color: #2ecc71;
}

input, textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid rgba(255,255,255,0.2);
  border-radius: 4px;
  font-family: inherit;
  box-sizing: border-box;
  background: rgba(0,0,0,0.3);
  color: #fff;
}

input:focus, textarea:focus {
  outline: none;
  border-color: #2ecc71;
}

input[readonly] {
  cursor: not-allowed;
  opacity: 0.7;
}

.choice-card {
  background: rgba(0,0,0,0.2);
  border: 1px solid rgba(255,255,255,0.1);
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

.scene-preview {
  width: 100%;
  min-height: 200px;
  max-height: 400px;
  background: rgba(0,0,0,0.4);
  border: 2px dashed rgba(46, 204, 113, 0.5);
  border-radius: 8px;
  margin: 1rem 0;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: relative;
}

.scene-preview img {
  width: 100%;
  height: auto;
  max-height: 400px;
  object-fit: contain;
}

.scene-preview.no-image {
  color: rgba(255,255,255,0.3);
  font-size: 0.9em;
}

.reaction-preview {
  margin-top: 0.5rem;
  max-width: 150px;
  max-height: 150px;
  border-radius: 4px;
  overflow: hidden;
  border: 1px solid rgba(255,255,255,0.2);
}

.reaction-preview img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

h3, h4 {
  color: #fff;
}
</style>