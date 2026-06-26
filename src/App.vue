<script setup>
import { ref, onMounted, watch } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { debounce } from 'lodash-es'
import { useProjectStore } from './stores/projectStore'

const store = useProjectStore()

const showLauncher = ref(true)
const recentProjects = ref([])
const showNewProjectModal = ref(false)
const newProjectName = ref('')
const bgExists = ref(false)
const editorBgPath = '/editor_bg.png'

const autoSave = debounce(async () => {
  await store.saveProject()
}, 1000)

watch(() => store.projectData, () => { autoSave() }, { deep: true })

async function checkBackground() {
  try {
    const img = new Image()
    img.src = editorBgPath
    await new Promise((resolve, reject) => {
      img.onload = resolve
      img.onerror = reject
    })
    bgExists.value = true
  } catch {
    bgExists.value = false
  }
}

async function loadRecentProjects() {
  try {
    recentProjects.value = await invoke('get_recent_projects')
  } catch (e) {
    console.error('Błąd ładowania ostatnich:', e)
    recentProjects.value = []
  }
}

async function openProjectDialog() {
  const selected = await open({
    directory: true,
    defaultPath: 'C:\\Users\\MD-Core\\OneDrive\\Dokumenty\\Janusz Projects'
  })
  if (selected) {
    await store.loadProject(selected)
    showLauncher.value = false
  }
}

async function createProject() {
  if (!newProjectName.value.trim()) {
    alert('Wpisz nazwę projektu')
    return
  }
  try {
    const path = await invoke('create_new_project', { name: newProjectName.value })
    await store.loadProject(path)
    showNewProjectModal.value = false
    newProjectName.value = ''
    showLauncher.value = false
    await loadRecentProjects()
  } catch (e) {
    alert('Błąd tworzenia: ' + e)
  }
}

function getAssetUrl(assetName) {
  if (!store.currentProjectPath ||!assetName) return '/assets/placeholders/no_bg.jpg'
  const fullPath = `${store.currentProjectPath}/assets/backgrounds/${assetName}.jpg`
  return convertFileSrc(fullPath)
}

function goToScene(sceneId) {
  store.currentSceneId = sceneId
}

function addScene() {
  const newId = `dzien_${store.projectData.scenes.length}`
  store.projectData.scenes.push({
    Id: newId,
    SceneTitle: 'Nowa scena',
    Background: '',
    Text: '',
    Choices: []
  })
  store.currentSceneId = newId
}

function deleteScene(sceneId) {
  if (!store.projectData || store.projectData.scenes.length <= 1) {
    alert('Nie możesz usunąć ostatniej sceny')
    return
  }
  store.projectData.scenes = store.projectData.scenes.filter(s => s.Id!== sceneId)
  if (store.currentSceneId === sceneId) {
    store.currentSceneId = store.projectData.scenes[0]?.Id
  }
}

function addChoice() {
  if (!store.currentScene) return
  if (!store.currentScene.Choices) store.currentScene.Choices = []
  store.currentScene.Choices.push({
    Text: 'Nowy wybór',
    Target: store.projectData.scenes[0]?.Id || ''
  })
}

function removeChoice(index) {
  if (!store.currentScene?.Choices) return
  store.currentScene.Choices.splice(index, 1)
}

onMounted(async () => {
  await checkBackground()
  await loadRecentProjects()
  showLauncher.value = true
  showNewProjectModal.value = false
})
</script>

<template>
  <div
    v-if="bgExists"
    class="bg-wrapper"
    :style="{ backgroundImage: `url(${editorBgPath})` }"
  ></div>

  <div class="app-container">
    <div v-if="showLauncher" class="launcher">
      <div class="launcher-content">
        <h1>Edytor Janusza V1.2</h1>
        <div class="launcher-buttons">
          <button @click="showNewProjectModal = true" class="btn-primary">+ Nowy Projekt</button>
          <button @click="openProjectDialog">📂 Otwórz Projekt</button>
        </div>
        <div v-if="recentProjects.length" class="recent">
          <h3>Ostatnie projekty:</h3>
          <div v-for="p in recentProjects" :key="p" @click="store.loadProject(p).then(() => showLauncher = false)" class="recent-item">
            {{ p.split('\\').pop() }}
          </div>
        </div>
      </div>
    </div>

    <div v-if="showNewProjectModal" class="modal" @click.self="showNewProjectModal = false">
      <div class="modal-content">
        <h3>Nowy projekt Janusza</h3>
        <input
          v-model="newProjectName"
          placeholder="Nazwa projektu"
          @keyup.enter="createProject"
          @keyup.esc="showNewProjectModal = false"
          autofocus
        />
        <div class="modal-buttons">
          <button @click="createProject" class="btn-primary">Stwórz</button>
          <button @click="showNewProjectModal = false">Anuluj</button>
        </div>
      </div>
    </div>

    <div v-else-if="!showLauncher" class="editor">
      <header class="top-bar">
        <h1>Edytor Janusza V1.2</h1>
        <div class="toolbar">
          <button @click="showLauncher = true">🏠 Menu</button>
          <button @click="store.saveProject" :disabled="!store.currentProjectPath">💾 Zapisz</button>
          <button @click="addScene">+ Scena</button>
          <span class="save-status">{{ store.saveStatus }}</span>
        </div>
        <div v-if="store.currentProjectPath" class="project-path">{{ store.currentProjectPath }}</div>
      </header>

      <div v-if="store.projectData" class="main-grid">
        <aside class="scenes-panel">
          <h3>Sceny [{{ store.projectData.scenes.length }}]</h3>
          <div v-for="scene in store.projectData.scenes" :key="scene.Id" :class="['scene-item', { active: scene.Id === store.currentSceneId }]" @click="goToScene(scene.Id)">
            <div class="scene-id">{{ scene.Id }}</div>
            <div class="scene-title">{{ scene.SceneTitle }}</div>
            <button @click.stop="deleteScene(scene.Id)" class="btn-delete-mini">✕</button>

  <div class="scene-id">{{ scene.Id }}</div>
  <div class="scene-title">{{ scene.SceneTitle }}</div>
  <button @click.stop="store.duplicateScene(scene.Id)" class="btn-duplicate-mini" title="Duplikuj">⧉</button>
  <button @click.stop="deleteScene(scene.Id)" class="btn-delete-mini">✕</button>
          </div>
        </aside>

        <main class="editor-panel">
          <div v-if="store.currentScene">
            <div class="bg-preview">
              <img v-if="store.currentScene.Background" :src="getAssetUrl(store.currentScene.Background)" alt="tło" />
              <div v-else class="no-bg">Brak tła</div>
              <div class="bg-label">{{ store.currentScene.Background || 'brak' }}</div>
            </div>
            <div class="scene-form">
              <div class="form-row"><label>ID Sceny:</label><input v-model="store.currentScene.Id" /></div>
              <div class="form-row"><label>Tytuł sceny:</label><input v-model="store.currentScene.SceneTitle" /></div>
              <div class="form-row">
                <label>Tło:</label>
                <select v-model="store.currentScene.Background">
                  <option value="">Brak</option>
                  <option v-for="bg in store.availableBackgrounds" :key="bg" :value="bg">{{ bg }}</option>
                </select>
              </div>
              <div class="form-row"><label>Tekst sceny:</label><textarea v-model="store.currentScene.Text" rows="8"></textarea></div>
            </div>
          </div>
        </main>

        <aside class="choices-panel">
          <div class="choices-header">
            <h3>Wybory</h3>
            <button @click="addChoice" class="btn-add">+ Dodaj</button>
          </div>
          <div v-if="!store.currentScene?.Choices || store.currentScene.Choices.length === 0" class="no-choices">Brak wyborów</div>
          <div v-for="(choice, idx) in store.currentScene?.Choices || []" :key="idx" class="choice-item">
            <div class="choice-edit">
              <input :value="choice.Text" @input="store.currentScene.Choices[idx].Text = $event.target.value" placeholder="Tekst wyboru" />
              <select :value="choice.Target" @change="store.currentScene.Choices[idx].Target = $event.target.value">
                <option v-for="s in store.projectData.scenes" :key="s.Id" :value="s.Id">{{ s.Id }} - {{ s.SceneTitle }}</option>
              </select>
            </div>
            <div class="choice-actions">
              <button @click="goToScene(choice.Target)">Idź →</button>
              <button @click="removeChoice(idx)" class="btn-delete">✕</button>
            </div>
          </div>
        </aside>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Twój CSS zostaw bez zmian */
.bg-wrapper {
  position: fixed;
  inset: 0;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  background-attachment: fixed;
  z-index: -2;
}
.bg-wrapper::after {
  content: '';
  position: absolute;
  inset: 0;
  background: rgba(10, 22, 40, 0.25);
  pointer-events: none;
}
.app-container {
  position: relative;
  min-height: 100vh;
  z-index: 1;
}
.launcher {
  position: fixed;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 200;
  background: rgba(10, 22, 40, 0.3);
}
.launcher-content { text-align: center; position: relative; z-index: 1; }
.launcher h1 { color: #4ade80; font-size: 48px; margin-bottom: 32px; text-shadow: 0 0 20px rgba(74, 222, 128, 0.5); }
.launcher-buttons { display: flex; flex-direction: column; gap: 12px; width: 300px; margin: 0 auto; }
.launcher-buttons button { padding: 16px; font-size: 16px; cursor: pointer; background: rgba(30, 41, 59, 0.9); border: 2px solid #334155; color: #fff; transition: all 0.2s; }
.launcher-buttons button:hover { background: rgba(51, 65, 85, 0.95); border-color: #16a34a; transform: translateY(-2px); }
.btn-primary { background: #16a34a!important; border-color: #16a34a!important; }
.btn-primary:hover { background: #22c55e!important; box-shadow: 0 0 15px rgba(34, 197, 94, 0.4); }
.recent { margin-top: 32px; }
.recent h3 { color: #94a3b8; font-size: 14px; margin-bottom: 12px; }
.recent-item { padding: 8px; background: rgba(30, 41, 59, 0.9); margin-bottom: 6px; cursor: pointer; font-family: monospace; font-size: 12px; }
.recent-item:hover { background: rgba(51, 65, 85, 0.95); color: #4ade80; }
.modal { position: fixed; inset: 0; background: rgba(0,0,0,0.8); display: flex; align-items: center; justify-content: center; z-index: 300; }
.modal-content { background: #1e293b; padding: 24px; border-radius: 8px; border: 2px solid #16a34a; min-width: 400px; }
.modal-content h3 { margin: 0 0 16px 0; color: #4ade80; }
.modal-content input { width: 100%; padding: 12px; margin: 12px 0; background: #0a1628; border: 1px solid #334155; color: #fff; font-size: 14px; }
.modal-content input:focus { outline: none; border-color: #16a34a; }
.modal-buttons { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.modal-buttons button { padding: 8px 16px; cursor: pointer; }
.editor {
  color: #fff;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
}
.top-bar {
  padding: 12px 20px;
  border-bottom: 2px solid #16a34a;
  background: rgba(30, 41, 59, 0.85);
}
.top-bar h1 { margin: 0 0 8px 0; font-size: 20px; text-shadow: 0 0 10px rgba(74, 222, 128, 0.3); }
.toolbar { display: flex; gap: 8px; align-items: center; }
.toolbar button { padding: 6px 12px; background: rgba(30, 41, 59, 0.95); border: 1px solid #334155; color: #fff; cursor: pointer; transition: all 0.2s; }
.toolbar button:hover { background: #16a34a; border-color: #16a34a; }
.toolbar button:disabled { opacity: 0.4; cursor: not-allowed; }
.save-status { color: #4ade80; font-size: 12px; margin-left: 12px; }
.project-path { font-size: 11px; color: #4ade80; margin-top: 6px; font-family: monospace; opacity: 0.7; }
.main-grid {
  display: grid;
  grid-template-columns: 250px 1fr 350px;
  gap: 1px;
  background: rgba(22, 163, 74, 0.2);
  flex: 1;
  overflow: hidden;
}
.scenes-panel,.editor-panel,.choices-panel {
  background: rgba(10, 22, 40, 0.75);
  padding: 16px;
  overflow-y: auto;
  border: 1px solid rgba(51, 65, 85, 0.3);
}
.scenes-panel h3,.choices-panel h3 { margin: 0 0 12px 0; color: #4ade80; font-size: 14px; }
.scene-item {
  padding: 10px;
  background: rgba(30, 41, 59, 0.8);
  margin-bottom: 6px;
  cursor: pointer;
  border-left: 3px solid transparent;
  position: relative;
  transition: all 0.2s;
}
.scene-item:hover { background: rgba(51, 65, 85, 0.9); border-left-color: #4ade80; }
.scene-item.active { background: rgba(22, 163, 74, 0.8); border-left-color: #4ade80; }
.scene-id { font-family: monospace; font-size: 11px; color: #94a3b8; }
.scene-title { font-size: 13px; margin-top: 2px; }
.btn-delete-mini { position: absolute; right: 6px; top: 6px; background: #dc2626; border: none; color: #fff; font-size: 10px; padding: 2px 6px; cursor: pointer; }
.bg-preview { position: relative; width: 100%; background: #000; border: 2px solid #334155; margin-bottom: 16px; }
.bg-preview img { width: 100%; height: auto; display: block; }
.bg-label { position: absolute; bottom: 8px; right: 8px; background: rgba(0,0,0,0.8); color: #4ade80; padding: 4px 8px; font-size: 11px; font-family: monospace; }
.no-bg { display: flex; align-items: center; justify-content: center; min-height: 200px; color: #64748b; font-style: italic; }
.scene-form { display: flex; flex-direction: column; gap: 12px; }
.form-row { display: flex; flex-direction: column; gap: 4px; }
.form-row label { color: #4ade80; font-size: 12px; font-weight: bold; }
.form-row input,.form-row select,.form-row textarea {
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid #334155;
  color: #fff;
  padding: 8px;
  font-family: inherit;
}
.form-row input:focus,.form-row select:focus,.form-row textarea:focus { outline: none; border-color: #16a34a; box-shadow: 0 0 5px rgba(22, 163, 74, 0.3); }
.choices-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.btn-add { padding: 4px 10px; background: #16a34a; border: none; color: #fff; cursor: pointer; font-size: 12px; }
.btn-add:hover { background: #22c55e; }
.choice-item { background: rgba(30, 41, 59, 0.8); padding: 12px; margin-bottom: 8px; border-left: 3px solid #16a34a; }
.choice-edit { display: flex; flex-direction: column; gap: 6px; margin-bottom: 8px; }
.choice-edit input,.choice-edit select { background: rgba(10, 22, 40, 0.95); border: 1px solid #334155; color: #fff; padding: 6px; font-size: 12px; }
.choice-actions { display: flex; gap: 6px; }
.choice-actions button { padding: 4px 8px; font-size: 11px; cursor: pointer; }
.btn-delete { background: #dc2626; border: none; color: #fff; }
.btn-delete:hover { background: #ef4444; }
.no-choices { color: #64748b; font-style: italic; text-align: center; padding: 20px; }

.btn-duplicate-mini { 
  position: absolute; 
  right: 28px; 
  top: 6px; 
  background: #2563eb; 
  border: none; 
  color: #fff; 
  font-size: 10px; 
  padding: 2px 6px; 
  cursor: pointer; 
}
.btn-duplicate-mini:hover { background: #3b82f6; }
</style>