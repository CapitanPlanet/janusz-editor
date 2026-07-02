<script setup>
import { ref, onMounted } from 'vue'
import { useProjectStore } from './stores/projectStore'
import LauncherView from './components/LauncherView.vue'
import EditorView from './components/EditorView.vue'

const store = useProjectStore()
const showLauncher = ref(true)
const bgExists = ref(false)
const editorBgPath = '/editor_bg.png'

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

function handleProjectLoaded() {
  showLauncher.value = false
}

onMounted(checkBackground)
</script>

<template>
  <div
    v-if="bgExists"
    class="bg-wrapper"
    :style="{ backgroundImage: `url(${editorBgPath})` }"
  ></div>

  <div class="app-container">
    <LauncherView 
      v-if="showLauncher" 
      @project-loaded="handleProjectLoaded" 
    />
    <EditorView 
      v-else 
      @go-to-menu="showLauncher = true" 
    />
  </div>
</template>

<style scoped>
.bg-wrapper {
  position: fixed;
  inset: 0;
  background-size: cover;
  background-position: center;
  z-index: -2;
}
.bg-wrapper::after {
  content: '';
  position: absolute;
  inset: 0;
  background: rgba(10, 22, 40, 0.25);
}
.app-container {
  position: relative;
  min-height: 100vh;
  z-index: 1;
}
</style>