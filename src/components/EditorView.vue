<script setup>
import SidePanel from './SidePanel.vue'
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
      <!-- ... top-bar bez zmian ... -->
    </header>

    <div v-if="store.projectData" class="main-grid">
      <SidePanel />      <!-- ← DODAJ TO -->
      <ScenesPanel />
      <EditorPanel />
      <ChoicesPanel />
    </div>
  </div>
</template>



<style scoped>
.editor {
  color: #fff;
  height: 100vh; /* ZMIANA: height zamiast min-height */
  display: flex;
  flex-direction: column;
  position: relative;
}
.main-grid {
  display: grid;
  grid-template-columns: 200px 250px 1fr 350px;  /* ← ZMIEŃ TO */
  gap: 1px;
  background: rgba(22, 163, 74, 0.2);
  flex: 1;
  overflow: hidden;
}

.top-bar {
  padding: 12px 20px;
  border-bottom: 2px solid #16a34a;
  background: rgba(30, 41, 59, 0.85);
  flex-shrink: 0; /* ZMIANA: nie ściskaj top-bara */
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

.project-path {
  font-size: 11px;
  color: #4ade80;
  margin-top: 6px;
  font-family: monospace;
  opacity: 0.7;
}

/* GRID NA 4 KOLUMNY: SidePanel + Sceny + Edytor + Wybory */
.main-grid {
  display: grid;
  grid-template-columns: 200px 250px 1fr 350px; /* ZMIANA: 4 kolumny */
  gap: 1px;
  background: rgba(22, 163, 74, 0.2);
  flex: 1;
  overflow: hidden; /* ZMIANA: każda kolumna scrolluje osobno */
  min-height: 0; /* ZMIANA: flex trick żeby działał overflow */
}
</style>