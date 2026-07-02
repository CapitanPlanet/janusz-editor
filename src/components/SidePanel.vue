<script setup>
import { useTabsStore } from '../stores/tabsStore'
import { useProjectStore } from '../stores/projectStore'

const tabs = useTabsStore()
const project = useProjectStore()
</script>

<template>
  <aside class="side-panel">
    <div class="project-header">PROJEKT JANUSZA</div>
    
    <div class="file-tree">
      <div class="folder">📁 Sceny</div>
      <div 
        v-for="scene in project.projectData?.scenes || []"
        :key="scene.Id"
        class="file"
        :title="`${scene.Id}.janusz`"
        @click="project.currentSceneId = scene.Id" 
      >
        📄 {{ scene.Id }}.janusz
      </div>
    </div>
  </aside>
</template>

<style scoped>
.side-panel {
  background: rgba(10, 22, 40, 0.9);
  padding: 16px 8px;
  border-right: 1px solid rgba(51, 65, 85, 0.5);
  overflow-y: auto;
  overflow-x: hidden;
}

.project-header {
  color: #4ade80;
  font-size: 12px;
  font-weight: bold;
  margin: 0 8px 16px 8px;
  letter-spacing: 1px;
}

.folder {
  color: #94a3b8;
  font-size: 11px;
  margin: 8px 8px 4px 8px;
  font-weight: bold;
  text-transform: uppercase;
}

.file {
  color: #e2e8f0;
  font-size: 12px;
  padding: 6px 8px;
  cursor: pointer;
  font-family: monospace;
  border-radius: 3px;
  margin: 0 4px;
  
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file:hover {
  background: rgba(51, 65, 85, 0.6);
  color: #4ade80;
}

/* WYJEBANE: wszystkie duplikaty poniżej były problemem */
</style>