<script setup>
import { ref, onMounted, computed } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'

const currentProjectPath = ref(null)
const projectData = ref(null)
const currentSceneId = ref(null)
const showNewProjectModal = ref(false)
const newProjectName = ref('')
const availableBackgrounds = ref([])

const currentScene = computed(() => {
  if (!projectData.value) return null
  return projectData.value.scenes.find(s => s.Id === currentSceneId.value)
})

const nextScenes = computed(() => {
  if (!currentScene.value?.Choices ||!projectData.value) return []
  return currentScene.value.Choices.map(choice => {
    const target = projectData.value.scenes.find(s => s.Id === choice.Target)
    return {
     ...choice,
      targetTitle: target?.SceneTitle || choice.Target,
      exists:!!target
    }
  })
})

// === PROJEKTY ===
async function loadProject(path) {
  try {
    const json = await invoke('load_project', { path })
    projectData.value = JSON.parse(json)
    currentProjectPath.value = path
    currentSceneId.value = projectData.value.scenes[0]?.Id || null
    await scanBackgrounds()
  } catch (e) {
    alert('Błąd wczytywania: ' + e)
  }
}

async function openProjectDialog() {
  const selected = await open({
    directory: true,
    defaultPath: await invoke('plugin:fs|document_dir')
  })
  if (selected) await loadProject(selected)
}

async function createProject() {
  if (!newProjectName.value) return
  try {
    const path = await invoke('create_new_project', { name: newProjectName.value })
    await loadProject(path)
    showNewProjectModal.value = false
    newProjectName.value = ''
  } catch (e) {
    alert('Błąd: ' + e)
  }
}

async function saveProject() {
  if (!currentProjectPath.value ||!projectData.value) return
  try {
    const json = JSON.stringify(projectData.value, null, 2)
    const path = `${currentProjectPath.value}/project.json`
    await invoke('plugin:fs|write_text_file', { path, contents: json })
    console.log('[PROJ] Zapisano')
  } catch (e) {
    alert('Błąd zapisu: ' + e)
  }
}

// === ASSSETY ===
function getAssetUrl(assetName) {
  if (!currentProjectPath.value ||!assetName) return ''
  const fullPath = `${currentProjectPath.value}/assets/backgrounds/${assetName}.jpg`
  return convertFileSrc(fullPath)
}

async function scanBackgrounds() {
  if (!currentProjectPath.value) return
  try {
    const bgPath = `${currentProjectPath.value}/assets/backgrounds`
    const entries = await invoke('plugin:fs|read_dir', { path: bgPath })
    availableBackgrounds.value = entries
     .filter(e => e.name.endsWith('.jpg') || e.name.endsWith('.png'))
     .map(e => e.name.replace(/\.[^/.]+$/, ""))
  } catch (e) {
    availableBackgrounds.value = []
  }
}

// === SCENY ===
function goToScene(sceneId) {
  currentSceneId.value = sceneId
}

function addChoice() {
  if (!currentScene.value) return
  if (!currentScene.value.Choices) currentScene.value.Choices = []
  currentScene.value.Choices.push({
    Text: 'Nowy wybór',
    Target: projectData.value.scenes[0]?.Id || ''
  })
}

function removeChoice(index) {
  if (!currentScene.value?.Choices) return
  currentScene.value.Choices.splice(index, 1)
}

onMounted(async () => {
  const last = await invoke('get_last_project')
  if (last) {
    await loadProject(last)
  } else {
    showNewProjectModal.value = true
  }
  
  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 's') {
      e.preventDefault()
      saveProject()
    }
  })
})
</script>

<template>
  <div class="editor">
    <header class="top-bar">
      <h1>Edytor Janusza V1.2</h1>
      <div class="toolbar">
        <button @click="showNewProjectModal = true">+ Nowy</button>
        <button @click="openProjectDialog">📂 Otwórz</button>
        <button @click="saveProject" :disabled="!currentProjectPath">💾 Zapisz</button>
      </div>
      <div v-if="currentProjectPath" class="project-path">{{ currentProjectPath }}</div>
    </header>

    <!-- Modal Nowy Projekt -->
    <div v-if="showNewProjectModal" class="modal">
      <div class="modal-content">
        <h3>Nowy projekt Janusza</h3>
        <input 
          v-model="newProjectName" 
          placeholder="Nazwa projektu" 
          @keyup.enter="createProject"
          autofocus
        />
        <button @click="createProject">Stwórz</button>
        <button @click="showNewProjectModal = false">Anuluj</button>
      </div>
    </div>

    <!-- Główny layout -->
    <div v-if="projectData" class="main-grid">
      
      <!-- LEWA: Lista scen -->
      <aside class="scenes-panel">
        <h3>Sceny [{{ projectData.scenes.length }}]</h3>
        <div 
          v-for="scene in projectData.scenes" 
          :key="scene.Id"
          :class="['scene-item', { active: scene.Id === currentSceneId }]"
          @click="goToScene(scene.Id)"
        >
          <div class="scene-id">{{ scene.Id }}</div>
          <div class="scene-title">{{ scene.SceneTitle }}</div>
        </div>
      </aside>

      <!-- ŚRODEK: Tło + Edycja -->
      <main class="editor-panel">
        <div v-if="currentScene">
          <!-- TŁO NA GÓRZE -->
          <div class="bg-preview">
            <img 
              v-if="currentScene.Background" 
              :src="getAssetUrl(currentScene.Background)" 
              alt="tło"
            />
            <div v-else class="no-bg">Brak tła</div>
            <div class="bg-label">{{ currentScene.Background || 'brak' }}</div>
          </div>

          <!-- DANE SCENY -->
          <div class="scene-form">
            <div class="form-row">
              <label>ID Sceny:</label>
              <input v-model="currentScene.Id" />
            </div>
            <div class="form-row">
              <label>Tytuł sceny:</label>
              <input v-model="currentScene.SceneTitle" />
            </div>
            <div class="form-row">
              <label>Tło:</label>
              <select v-model="currentScene.Background">
                <option value="">Brak</option>
                <option v-for="bg in availableBackgrounds" :key="bg" :value="bg">{{ bg }}</option>
              </select>
            </div>
            <div class="form-row">
              <label>Tekst sceny:</label>
              <textarea v-model="currentScene.Text" rows="8"></textarea>
            </div>
          </div>
        </div>
      </main>

      <!-- PRAWA: Wybory -->
      <aside class="choices-panel">
        <div class="choices-header">
          <h3>Wybory</h3>
          <button @click="addChoice" class="btn-add">+ Dodaj</button>
        </div>
        
        <div v-if="nextScenes.length === 0" class="no-choices">
          Brak wyborów - koniec ścieżki
        </div>
        
        <div 
          v-for="(choice, idx) in nextScenes" 
          :key="idx"
          :class="['choice-item', { missing:!choice.exists }]"
        >
          <div class="choice-edit">
            <input v-model="choice.Text" placeholder="Tekst wyboru" />
            <select v-model="choice.Target">
              <option v-for="s in projectData.scenes" :key="s.Id" :value="s.Id">
                {{ s.Id }} - {{ s.SceneTitle }}
              </option>
            </select>
          </div>
          <div class="choice-actions">
            <button @click="choice.exists && goToScene(choice.Target)" :disabled="!choice.exists">
              Idź →
            </button>
            <button @click="removeChoice(idx)" class="btn-delete">✕</button>
          </div>
          <div v-if="!choice.exists" class="missing-badge">SCENA NIE ISTNIEJE</div>
        </div>
      </aside>

    </div>
  </div>
</template>

<style scoped>
.editor { background: #0a1628; color: #fff; min-height: 100vh; display: flex; flex-direction: column; }

.top-bar { padding: 12px 20px; border-bottom: 2px solid #16a34a; }
.top-bar h1 { margin: 0 0 8px 0; font-size: 20px; }
.toolbar { display: flex; gap: 8px; }
.toolbar button { padding: 6px 12px; background: #1e293b; border: 1px solid #334155; color: #fff; cursor: pointer; }
.toolbar button:hover { background: #334155; }
.toolbar button:disabled { opacity: 0.4; cursor: not-allowed; }
.project-path { font-size: 11px; color: #4ade80; margin-top: 6px; font-family: monospace; }

.modal { position: fixed; inset: 0; background: rgba(0,0,0,0.8); display: flex; align-items: center; justify-content: center; z-index: 100; }
.modal-content { background: #1e293b; padding: 24px; border-radius: 8px; border: 2px solid #16a34a; }
.modal-content input { width: 300px; padding: 8px; margin: 12px 0; background: #0a1628; border: 1px solid #334155; color: #fff; }
.modal-content button { margin-right: 8px; padding: 6px 12px; }

.main-grid { 
  display: grid; 
  grid-template-columns: 250px 1fr 350px; 
  gap: 1px; 
  background: #16a34a;
  flex: 1;
  overflow: hidden;
}

.scenes-panel,.editor-panel,.choices-panel { 
  background: #0a1628; 
  padding: 16px; 
  overflow-y: auto;
}

.scenes-panel h3,.choices-panel h3 { margin: 0 0 12px 0; color: #4ade80; font-size: 14px; }

.scene-item { 
  padding: 10px; 
  background: #1e293b; 
  margin-bottom: 6px; 
  cursor: pointer; 
  border-left: 3px solid transparent;
}
.scene-item:hover { background: #334155; }
.scene-item.active { background: #16a34a; border-left-color: #4ade80; }
.scene-id { font-family: monospace; font-size: 11px; color: #94a3b8; }
.scene-title { font-size: 13px; margin-top: 2px; }

.bg-preview {
  position: relative;
  width: 100%;
  background: #000;
  border: 2px solid #334155;
  margin-bottom: 16px;
}

.bg-preview img {
  width: 100%;
  height: auto;
  display: block;
}

.bg-label {
  position: absolute;
  bottom: 8px;
  right: 8px;
  background: rgba(0,0,0,0.8);
  color: #4ade80;
  padding: 4px 8px;
  font-size: 11px;
  font-family: monospace;
}

.no-bg {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 200px;
  color: #64748b;
  font-style: italic;
}

.no-bg {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #64748b;
  font-style: italic;
}

.scene-form { display: flex; flex-direction: column; gap: 12px; }
.form-row { display: flex; flex-direction: column; gap: 4px; }
.form-row label { color: #4ade80; font-size: 12px; font-weight: bold; }
.form-row input,.form-row select,.form-row textarea {
  background: #1e293b;
  border: 1px solid #334155;
  color: #fff;
  padding: 8px;
  font-family: inherit;
}
.form-row input:focus,.form-row select:focus,.form-row textarea:focus {
  outline: none;
  border-color: #16a34a;
}

.choices-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.btn-add { padding: 4px 10px; background: #16a34a; border: none; color: #fff; cursor: pointer; font-size: 12px; }
.btn-add:hover { background: #22c55e; }

.choice-item {
  background: #1e293b;
  padding: 12px;
  margin-bottom: 8px;
  border-left: 3px solid #16a34a;
}
.choice-item.missing { border-left-color: #ef4444; opacity: 0.7; }
.choice-edit { display: flex; flex-direction: column; gap: 6px; margin-bottom: 8px; }
.choice-edit input,.choice-edit select {
  background: #0a1628;
  border: 1px solid #334155;
  color: #fff;
  padding: 6px;
  font-size: 12px;
}
.choice-actions { display: flex; gap: 6px; }
.choice-actions button { padding: 4px 8px; font-size: 11px; cursor: pointer; }
.btn-delete { background: #dc2626; border: none; color: #fff; }
.btn-delete:hover { background: #ef4444; }
.missing-badge { background: #ef4444; color: #fff; padding: 2px 6px; border-radius: 3px; margin-top: 6px; font-size: 10px; display: inline-block; }
.no-choices { color: #64748b; font-style: italic; text-align: center; padding: 20px; }
</style>