<script setup>
import ScenesPanel from './ScenesPanel.vue'
import EditorPanel from './EditorPanel.vue' 
import ChoicesPanel from './ChoicesPanel.vue'
import { useProjectStore } from '../stores/projectStore'


const store = useProjectStore()
const emit = defineEmits(['go-to-menu'])
</script>

<template>
  <div class="editor">
    <header class="top-bar">
      <h1>Edytor Janusza V1.2</h1>
      <div class="toolbar">
        <button @click="emit('go-to-menu')">🏠 Menu</button>
        <button @click="store.saveProject">💾 Zapisz</button>
        <span class="save-status">{{ store.saveStatus }}</span>
      </div>
    </header>

    <div v-if="store.projectData" class="main-grid">
      <ScenesPanel />
      <EditorPanel />
      <ChoicesPanel />
    </div>
  </div>
</template>

<style scoped>
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

.top-bar h1 {
  margin: 0 0 8px 0;
  font-size: 20px;
  text-shadow: 0 0 10px rgba(74, 222, 128, 0.3);
}

.toolbar {
  display: flex;
  gap: 8px;
  align-items: center;
}

.toolbar button {
  padding: 6px 12px;
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid #334155;
  color: #fff;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar button:hover {
  background: #16a34a;
  border-color: #16a34a;
}

.save-status {
  color: #4ade80;
  font-size: 12px;
  margin-left: 12px;
}

/* TO JEST KLUCZOWE - GRID NA 3 KOLUMNY */
.main-grid {
  display: grid;
  grid-template-columns: 250px 1fr 350px;
  gap: 1px;
  background: rgba(22, 163, 74, 0.2);
  flex: 1;
  overflow: hidden;
}
</style>