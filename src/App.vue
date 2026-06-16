<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readDir } from '@tauri-apps/plugin-fs'
import { debounce } from 'lodash-es'

const showLauncher = ref(true)
const recentProjects = ref<string[]>([])
const currentProjectPath = ref<string | null>(null)
const projectData = ref<any>(null)
const currentSceneId = ref<string | null>(null)
const showNewProjectModal = ref(false)
const newProjectName = ref('')
const availableBackgrounds = ref<string[]>([])
const saveStatus = ref('')

const currentScene = computed(() => {
  if (!projectData.value) return null
  return projectData.value.scenes.find((s: any) => s.Id === currentSceneId.value)
})

// === AUTO-SAVE ===
const autoSave = debounce(async () => {
  if (!currentProjectPath.value ||!projectData.value) return
  saveStatus.value = 'Zapisywanie...'
  try {
    const cleanData = JSON.parse(JSON.stringify(projectData.value))
    const json = JSON.stringify(cleanData, null, 2)
    const path = `${currentProjectPath.value}/project.json`
    await writeTextFile(path, json)
    saveStatus.value = 'Zapisano'
    setTimeout(() => { saveStatus.value = '' }, 2000)
    console.log('[AUTO-SAVE]', path)
  } catch (e) {
    saveStatus.value = 'Błąd zapisu!'
    console.error('[AUTO-SAVE]', e)
  }
}, 1000)

watch(projectData, () => { autoSave() }, { deep: true })

// === LAUNCHER ===
async function loadRecentProjects() {
  try {
    recentProjects.value = await invoke('get_recent_projects')
  } catch (e) {
    console.error('Błąd ładowania ostatnich:', e)
    recentProjects.value = []
  }
}

// === PROJEKTY ===
async function loadProject(path: string) {
  try {
    const json = await invoke<string>('load_project', { path })
    projectData.value = JSON.parse(json)
    currentProjectPath.value = path
    currentSceneId.value = projectData.value.scenes[0]?.Id || null
    showLauncher.value = false
    await scanBackgrounds()
    console.log('[PROJ] Wczytano:', path)
  } catch (e) {
    alert('Błąd wczytywania: ' + e)
  }
}

async function openProjectDialog() {
  const selected = await open({
    directory: true,
    defaultPath: 'C:\\Users\\MD-Core\\OneDrive\\Dokumenty\\Janusz Projects'
  })
  if (selected) await loadProject(selected as string)
}

async function createProject() {
  if (!newProjectName.value.trim()) return
  try {
    const path = await invoke<string>('create_new_project', { name: newProjectName.value })
    await loadProject(path)
    showNewProjectModal.value = false
    newProjectName.value = ''
    await loadRecentProjects()
  } catch (e) {
    alert('Błąd: ' + e)
  }
}

async function saveProject() {
  if (!currentProjectPath.value ||!projectData.value) return
  saveStatus.value = 'Zapisywanie...'
  try {
    const cleanData = JSON.parse(JSON.stringify(projectData.value))
    const json = JSON.stringify(cleanData, null, 2)
    const path = `${currentProjectPath.value}/project.json`
    await writeTextFile(path, json)
    saveStatus.value = 'Zapisano!'
    setTimeout(() => { saveStatus.value = '' }, 2000)
  } catch (e) {
    saveStatus.value = 'Błąd!'
    alert('Błąd zapisu: ' + e)
  }
}

// === TŁA ===
function getAssetUrl(assetName: string) {
  if (!currentProjectPath.value ||!assetName) return ''
  const fullPath = `${currentProjectPath.value}/assets/backgrounds/${assetName}.jpg`
  return convertFileSrc(fullPath)
}

async function scanBackgrounds() {
  if (!currentProjectPath.value) return
  try {
    const bgPath = `${currentProjectPath.value}/assets/backgrounds`
    const entries = await readDir(bgPath)
    availableBackgrounds.value = entries
     .filter(e => e.name.endsWith('.jpg') || e.name.endsWith('.png'))
     .map(e => e.name.replace(/\.[^/.]+$/, ""))
  } catch (e) {
    console.error('[PROJ] Błąd skanowania teł:', e)
    availableBackgrounds.value = []
  }
}

// === SCENY ===
function goToScene(sceneId: string) {
  currentSceneId.value = sceneId
}

function addScene() {
  if (!projectData.value) return
  const newId = `dzien_${projectData.value.scenes.length}`
  projectData.value.scenes.push({
    Id: newId,
    SceneTitle: 'Nowa scena',
    Background: '',
    Text: '',
    Choices: []
  })
  currentSceneId.value = newId
}

function deleteScene(sceneId: string) {
  if (!projectData.value || projectData.value.scenes.length <= 1) {
    alert('Nie możesz usunąć ostatniej sceny')
    return
  }
  projectData.value.scenes = projectData.value.scenes.filter((s: any) => s.Id!== sceneId)
  if (currentSceneId.value === sceneId) {
    currentSceneId.value = projectData.value.scenes[0]?.Id
  }
}

function addChoice() {
  if (!currentScene.value) return
  if (!currentScene.value.Choices) currentScene.value.Choices = []
  currentScene.value.Choices.push({
    Text: 'Nowy wybór',
    Target: projectData.value.scenes[0]?.Id || ''
  })
}

function removeChoice(index: number) {
  if (!currentScene.value?.Choices) return
  currentScene.value.Choices.splice(index, 1)
}

onMounted(async () => {
  await loadRecentProjects()
  showLauncher.value = true
})
</script>

<template>
  <!-- LAUNCHER -->
  <div v-if="showLauncher" class="launcher">
    <div class="launcher-content">
      <h1>Edytor Janusza V1.2</h1>
      <div class="launcher-buttons">
        <button @click="showNewProjectModal = true" class="btn-primary">+ Nowy Projekt</button>
        <button @click="openProjectDialog">📂 Otwórz Projekt</button>
      </div>
      <div v-if="recentProjects.length" class="recent">
        <h3>Ostatnie projekty:</h3>
        <div v-for="p in recentProjects" :key="p" @click="loadProject(p)" class="recent-item">
          {{ p.split('\\').pop() }}
        </div>
      </div>
    </div>
  </div>

  <!-- EDYTOR -->
  <div v-else class="editor">
    <header class="top-bar">
      <h1>Edytor Janusza V1.2</h1>
      <div class="toolbar">
        <button @click="showLauncher = true">🏠 Menu</button>
        <button @click="saveProject" :disabled="!currentProjectPath">💾 Zapisz</button>
        <button @click="addScene">+ Scena</button>
        <span class="save-status">{{ saveStatus }}</span>
      </div>
      <div v-if="currentProjectPath" class="project-path">{{ currentProjectPath }}</div>
    </header>

    <div v-if="showNewProjectModal" class="modal">
      <div class="modal-content">
        <h3>Nowy projekt Janusza</h3>
        <input v-model="newProjectName" placeholder="Nazwa projektu" @keyup.enter="createProject" autofocus />
        <button @click="createProject">Stwórz</button>
        <button @click="showNewProjectModal = false">Anuluj</button>
      </div>
    </div>

    <div v-if="projectData" class="main-grid">
      <aside class="scenes-panel">
        <h3>Sceny [{{ projectData.scenes.length }}]</h3>
        <div v-for="scene in projectData.scenes" :key="scene.Id" :class="['scene-item', { active: scene.Id === currentSceneId }]" @click="goToScene(scene.Id)">
          <div class="scene-id">{{ scene.Id }}</div>
          <div class="scene-title">{{ scene.SceneTitle }}</div>
          <button @click.stop="deleteScene(scene.Id)" class="btn-delete-mini">✕</button>
        </div>
      </aside>

      <main class="editor-panel">
        <div v-if="currentScene">
          <div class="bg-preview">
            <img v-if="currentScene.Background" :src="getAssetUrl(currentScene.Background)" alt="tło" />
            <div v-else class="no-bg">Brak tła</div>
            <div class="bg-label">{{ currentScene.Background || 'brak' }}</div>
          </div>
          <div class="scene-form">
            <div class="form-row"><label>ID Sceny:</label><input v-model="currentScene.Id" /></div>
            <div class="form-row"><label>Tytuł sceny:</label><input v-model="currentScene.SceneTitle" /></div>
            <div class="form-row">
              <label>Tło:</label>
              <select v-model="currentScene.Background">
                <option value="">Brak</option>
                <option v-for="bg in availableBackgrounds" :key="bg" :value="bg">{{ bg }}</option>
              </select>
            </div>
            <div class="form-row"><label>Tekst sceny:</label><textarea v-model="currentScene.Text" rows="8"></textarea></div>
          </div>
        </div>
      </main>

      <aside class="choices-panel">
        <div class="choices-header">
          <h3>Wybory</h3>
          <button @click="addChoice" class="btn-add">+ Dodaj</button>
        </div>
        <div v-if="!currentScene?.Choices || currentScene.Choices.length === 0" class="no-choices">Brak wyborów</div>
        <div v-for="(choice, idx) in currentScene?.Choices || []" :key="idx" class="choice-item">
          <div class="choice-edit">
            <input :value="choice.Text" @input="currentScene.Choices[idx].Text = ($event.target as HTMLInputElement).value" placeholder="Tekst wyboru" />
            <select :value="choice.Target" @change="currentScene.Choices[idx].Target = ($event.target as HTMLSelectElement).value">
              <option v-for="s in projectData.scenes" :key="s.Id" :value="s.Id">{{ s.Id }} - {{ s.SceneTitle }}</option>
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
</template>

<style scoped>
.launcher { position: fixed; inset: 0; background: #0a1628; display: flex; align-items: center; justify-content: center; z-index: 200; }
.launcher-content { text-align: center; }
.launcher h1 { color: #4ade80; font-size: 32px; margin-bottom: 32px; }
.launcher-buttons { display: flex; flex-direction: column; gap: 12px; width: 300px; margin: 0 auto; }
.launcher-buttons button { padding: 16px; font-size: 16px; cursor: pointer; background: #1e293b; border: 2px solid #334155; color: #fff; }
.launcher-buttons button:hover { background: #334155; border-color: #16a34a; }
.btn-primary { background: #16a34a!important; border-color: #16a34a!important; }
.btn-primary:hover { background: #22c55e!important; }
.recent { margin-top: 32px; }
.recent h3 { color: #94a3b8; font-size: 14px; margin-bottom: 12px; }
.recent-item { padding: 8px; background: #1e293b; margin-bottom: 6px; cursor: pointer; font-family: monospace; font-size: 12px; }
.recent-item:hover { background: #334155; color: #4ade80; }

.editor { background: #0a1628; color: #fff; min-height: 100vh; display: flex; flex-direction: column; }
.top-bar { padding: 12px 20px; border-bottom: 2px solid #16a34a; }
.top-bar h1 { margin: 0 0 8px 0; font-size: 20px; }
.toolbar { display: flex; gap: 8px; align-items: center; }
.toolbar button { padding: 6px 12px; background: #1e293b; border: 1px solid #334155; color: #fff; cursor: pointer; }
.toolbar button:hover { background: #334155; }
.toolbar button:disabled { opacity: 0.4; cursor: not-allowed; }
.save-status { color: #4ade80; font-size: 12px; margin-left: 12px; }
.project-path { font-size: 11px; color: #4ade80; margin-top: 6px; font-family: monospace; }
.modal { position: fixed; inset: 0; background: rgba(0,0,0,0.8); display: flex; align-items: center; justify-content: center; z-index: 100; }
.modal-content { background: #1e293b; padding: 24px; border-radius: 8px; border: 2px solid #16a34a; }
.modal-content input { width: 300px; padding: 8px; margin: 12px 0; background: #0a1628; border: 1px solid #334155; color: #fff; }
.modal-content button { margin-right: 8px; padding: 6px 12px; }
.main-grid { display: grid; grid-template-columns: 250px 1fr 350px; gap: 1px; background: #16a34a; flex: 1; overflow: hidden; }
.scenes-panel,.editor-panel,.choices-panel { background: #0a1628; padding: 16px; overflow-y: auto; }
.scenes-panel h3,.choices-panel h3 { margin: 0 0 12px 0; color: #4ade80; font-size: 14px; }
.scene-item { padding: 10px; background: #1e293b; margin-bottom: 6px; cursor: pointer; border-left: 3px solid transparent; position: relative; }
.scene-item:hover { background: #334155; }
.scene-item.active { background: #16a34a; border-left-color: #4ade80; }
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
.form-row input,.form-row select,.form-row textarea { background: #1e293b; border: 1px solid #334155; color: #fff; padding: 8px; font-family: inherit; }
.form-row input:focus,.form-row select:focus,.form-row textarea:focus { outline: none; border-color: #16a34a; }
.choices-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.btn-add { padding: 4px 10px; background: #16a34a; border: none; color: #fff; cursor: pointer; font-size: 12px; }
.btn-add:hover { background: #22c55e; }
.choice-item { background: #1e293b; padding: 12px; margin-bottom: 8px; border-left: 3px solid #16a34a; }
.choice-edit { display: flex; flex-direction: column; gap: 6px; margin-bottom: 8px; }
.choice-edit input,.choice-edit select { background: #0a1628; border: 1px solid #334155; color: #fff; padding: 6px; font-size: 12px; }
.choice-actions { display: flex; gap: 6px; }
.choice-actions button { padding: 4px 8px; font-size: 11px; cursor: pointer; }
.btn-delete { background: #dc2626; border: none; color: #fff; }
.btn-delete:hover { background: #ef4444; }
.no-choices { color: #64748b; font-style: italic; text-align: center; padding: 20px; }
</style>