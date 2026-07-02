<script setup lang="ts">
import { useTabsStore } from '@/stores/tabsStore'
import { useProjectStore } from '@/stores/projectStore'

const tabs = useTabsStore()
const project = useProjectStore()

// Zakładam że project.scenes to Twoje pliki/sceny
</script>

<template>
  <div class="side-panel">
    <div class="panel-header">PROJEKT JANUSZA</div>
    
    <div 
      v-for="scene in project.scenes" 
      :key="scene.id"
      class="file-item"
      :class="{ active: tabs.activeFile === scene.path }"
      @click="tabs.openFile(scene.path)"
    >
      <span class="icon">📄</span>
      {{ scene.name }}.janusz
      <button 
        v-if="tabs.openFiles.includes(scene.path)"
        class="close-btn" 
        @click.stop="tabs.closeFile(scene.path)"
      >×</button>
    </div>
  </div>
</template>

<style scoped>
.side-panel {
  width: 250px;
  background: #252526;
  color: #ccc;
  height: 100vh;
  border-right: 1px solid #1e1e1e;
}
.panel-header {
  padding: 8px 12px;
  font-size: 11px;
  font-weight: bold;
  color: #888;
}
.file-item {
  padding: 4px 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}
.file-item:hover { background: #2a2d2e; }
.file-item.active { background: #37373d; }
.close-btn {
  margin-left: auto;
  background: none;
  border: none;
  color: #888;
  cursor: pointer;
}
.close-btn:hover { color: #fff; }
</style>