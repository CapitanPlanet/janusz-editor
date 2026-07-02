<template>
  <div class="topbar">
    <div class="actions">
      <button @click="newProject" title="Ctrl+N">📄 Nowy</button>
      <button @click="openProject" title="Ctrl+O">📂 Otwórz</button>
      <button @click="saveProject" title="Ctrl+S">💾 Zapisz</button>
      <button @click="saveAsProject">💾 Zapisz jako...</button>
    </div>
    <div class="title">
      Edytor Janusza V1.3 
      <span v-if="projectStore.currentFileName">- {{ projectStore.currentFileName }}</span>
    </div>
    <div class="right">
      <button @click="buildGame">▶️ Build</button>
    </div>
  </div>
</template>

<script setup>
import { useProjectStore } from '../stores/projectStore'

const projectStore = useProjectStore()

const newProject = () => projectStore.newProject()
const openProject = () => projectStore.openProject()
const saveProject = () => projectStore.saveProject()
const saveAsProject = () => projectStore.saveAsProject()
const buildGame = () => projectStore.buildGame()
</script>

<style scoped>
.topbar {
  height: 36px;
  background: #2d2d2d;
  border-bottom: 1px solid #1e1e1e;
  display: flex;
  align-items: center;
  padding: 0 8px;
  user-select: none;
}
.actions {
  display: flex;
  gap: 4px;
}
button {
  background: transparent;
  border: 1px solid transparent;
  color: #cccccc;
  padding: 4px 8px;
  border-radius: 3px;
  cursor: pointer;
  font-size: 13px;
}
button:hover {
  background: #3c3c3c;
  border-color: #555;
}
.title {
  flex: 1;
  text-align: center;
  color: #999;
  font-size: 12px;
}
.right {
  display: flex;
  gap: 4px;
}
</style>