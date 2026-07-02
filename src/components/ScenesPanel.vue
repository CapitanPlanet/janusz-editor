<script setup>
import { useProjectStore } from '../stores/projectStore'

const store = useProjectStore()

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
</script>

<template>
  <aside class="scenes-panel">
    <h3>Sceny [{{ store.projectData?.scenes.length || 0 }}]</h3>
    <button @click="addScene" class="btn-add">+ Scena</button>

    <div
      v-for="scene in store.projectData?.scenes || []"
      :key="scene.Id"
      :class="['scene-item', { active: scene.Id === store.currentSceneId }]"
      @click="goToScene(scene.Id)"
    >
      <div class="scene-id">{{ scene.Id }}</div>
      <div class="scene-title">{{ scene.SceneTitle }}</div>
      <button @click.stop="deleteScene(scene.Id)" class="btn-delete-mini">✕</button>
    </div>
  </aside>
</template>

<style scoped>
.scenes-panel {
  background: rgba(10, 22, 40, 0.75);
  padding: 16px;
  overflow-y: auto;
  border: 1px solid rgba(51, 65, 85, 0.3);
}
.scenes-panel h3 {
  margin: 0 0 12px 0;
  color: #4ade80;
  font-size: 14px;
}
.btn-add {
  padding: 4px 10px;
  background: #16a34a;
  border: none;
  color: #fff;
  cursor: pointer;
  font-size: 12px;
  margin-bottom: 12px;
  width: 100%;
}
.btn-add:hover { background: #22c55e; }
.scene-item {
  padding: 10px;
  background: rgba(30, 41, 59, 0.8);
  margin-bottom: 6px;
  cursor: pointer;
  border-left: 3px solid transparent;
  position: relative;
  transition: all 0.2s;
}
.scene-item:hover {
  background: rgba(51, 65, 85, 0.9);
  border-left-color: #4ade80;
}
.scene-item.active {
  background: rgba(22, 163, 74, 0.8);
  border-left-color: #4ade80;
}
.scene-id {
  font-family: monospace;
  font-size: 11px;
  color: #94a3b8;
}
.scene-title {
  font-size: 13px;
  margin-top: 2px;
  color: #fff;
}
.btn-delete-mini {
  position: absolute;
  right: 6px;
  top: 6px;
  background: #dc2626;
  border: none;
  color: #fff;
  font-size: 10px;
  padding: 2px 6px;
  cursor: pointer;
}
</style>