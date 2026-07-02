<script setup>
import { useProjectStore } from '../stores/projectStore'

const store = useProjectStore()

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

function goToScene(sceneId) {
  store.currentSceneId = sceneId
}
</script>

<template>
  <aside class="choices-panel">
    <div class="choices-header">
      <h3>Wybory</h3>
      <button @click="addChoice" class="btn-add">+ Dodaj</button>
    </div>

    <div
      v-if="!store.currentScene?.Choices || store.currentScene.Choices.length === 0"
      class="no-choices"
    >
      Brak wyborów
    </div>

    <div
      v-for="(choice, idx) in store.currentScene?.Choices || []"
      :key="idx"
      class="choice-item"
    >
      <div class="choice-edit">
        <input
          v-model="store.currentScene.Choices[idx].Text"
          placeholder="Tekst wyboru"
        />
        <select v-model="store.currentScene.Choices[idx].Target">
          <option
            v-for="s in store.projectData.scenes"
            :key="s.Id"
            :value="s.Id"
          >
            {{ s.Id }} - {{ s.SceneTitle }}
          </option>
        </select>
      </div>
      <div class="choice-actions">
        <button @click="goToScene(choice.Target)">Idź →</button>
        <button @click="removeChoice(idx)" class="btn-delete">✕</button>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.choices-panel {
  background: rgba(10, 22, 40, 0.75);
  padding: 16px;
  overflow-y: auto;
  border: 1px solid rgba(51, 65, 85, 0.3);
}
.choices-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}
.choices-header h3 {
  margin: 0;
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
}
.btn-add:hover { background: #22c55e; }
.choice-item {
  background: rgba(30, 41, 59, 0.8);
  padding: 12px;
  margin-bottom: 8px;
  border-left: 3px solid #16a34a;
}
.choice-edit {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 8px;
}
.choice-edit input,
.choice-edit select {
  background: rgba(10, 22, 40, 0.95);
  border: 1px solid #334155;
  color: #fff;
  padding: 6px;
  font-size: 12px;
}
.choice-actions {
  display: flex;
  gap: 6px;
}
.choice-actions button {
  padding: 4px 8px;
  font-size: 11px;
  cursor: pointer;
  background: rgba(30, 41, 59, 0.95);
  border: 1px solid #334155;
  color: #fff;
}
.btn-delete {
  background: #dc2626!important;
  border: none!important;
  color: #fff;
}
.btn-delete:hover { background: #ef4444!important; }
.no-choices {
  color: #64748b;
  font-style: italic;
  text-align: center;
  padding: 20px;
}
</style>
