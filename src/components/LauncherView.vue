<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { useProjectStore } from '../stores/projectStore'

const store = useProjectStore()
const emit = defineEmits(['project-loaded'])

const recentProjects = ref([])
const showNewProjectModal = ref(false)
const newProjectName = ref('')

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
    emit('project-loaded')
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
    emit('project-loaded')
    await loadRecentProjects()
  } catch (e) {
    alert('Błąd tworzenia: ' + e)
  }
}

onMounted(loadRecentProjects)
</script>

<template>
  <div class="launcher">
    <div class="launcher-content">
      <h1>Edytor Janusza V1.2</h1>
      <div class="launcher-buttons">
        <button @click="showNewProjectModal = true" class="btn-primary">+ Nowy Projekt</button>
        <button @click="openProjectDialog">📂 Otwórz Projekt</button>
      </div>
      <div v-if="recentProjects.length" class="recent">
        <h3>Ostatnie projekty:</h3>
        <div
          v-for="p in recentProjects"
          :key="p"
          @click="store.loadProject(p).then(() => emit('project-loaded'))"
          class="recent-item"
        >
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
</template>

<style scoped>
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
.launcher h1 { 
  color: #4ade80; 
  font-size: 48px; 
  margin-bottom: 32px; 
  text-shadow: 0 0 20px rgba(74, 222, 128, 0.5); 
}
.launcher-buttons { 
  display: flex; 
  flex-direction: column; 
  gap: 12px; 
  width: 300px; 
  margin: 0 auto; 
}
.launcher-buttons button { 
  padding: 16px; 
  font-size: 16px; 
  cursor: pointer; 
  background: rgba(30, 41, 59, 0.9); 
  border: 2px solid #334155; 
  color: #fff; 
  transition: all 0.2s; 
}
.launcher-buttons button:hover { 
  background: rgba(51, 65, 85, 0.95); 
  border-color: #16a34a; 
  transform: translateY(-2px); 
}
.btn-primary { 
  background: #16a34a!important; 
  border-color: #16a34a!important; 
}
.btn-primary:hover { 
  background: #22c55e!important; 
  box-shadow: 0 0 15px rgba(34, 197, 94, 0.4); 
}
.recent { margin-top: 32px; }
.recent h3 { color: #94a3b8; font-size: 14px; margin-bottom: 12px; }
.recent-item { 
  padding: 8px; 
  background: rgba(30, 41, 59, 0.9); 
  margin-bottom: 6px; 
  cursor: pointer; 
  font-family: monospace; 
  font-size: 12px; 
}
.recent-item:hover { 
  background: rgba(51, 65, 85, 0.95); 
  color: #4ade80; 
}
.modal { 
  position: fixed; 
  inset: 0; 
  background: rgba(0,0,0,0.8); 
  display: flex; 
  align-items: center; 
  justify-content: center; 
  z-index: 300; 
}
.modal-content { 
  background: #1e293b; 
  padding: 24px; 
  border-radius: 8px; 
  border: 2px solid #16a34a; 
  min-width: 400px; 
}
.modal-content h3 { margin: 0 0 16px 0; color: #4ade80; }
.modal-content input { 
  width: 100%; 
  padding: 12px; 
  margin: 12px 0; 
  background: #0a1628; 
  border: 1px solid #334155; 
  color: #fff; 
  font-size: 14px; 
}
.modal-content input:focus { outline: none; border-color: #16a34a; }
.modal-buttons { display: flex; gap: 8px; justify-content: flex-end; margin-top: 16px; }
.modal-buttons button { padding: 8px 16px; cursor: pointer; }
</style>