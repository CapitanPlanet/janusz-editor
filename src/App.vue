<script setup>
import { ref, onMounted } from 'vue'
import { useProjectStore } from './stores/projectStore'
import LauncherView from './components/LauncherView.vue'
import EditorView from './components/EditorView.vue'
import TopBar from './components/TopBar.vue' // ← DODAJ

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

function goToMenu() {
  showLauncher.value = true
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
    <!-- Launcher bez TopBara -->
    <LauncherView 
      v-if="showLauncher" 
      @project-loaded="handleProjectLoaded" 
    />
    
    <!-- Edytor Z TopBarem -->
    <template v-else>
      <TopBar @go-to-menu="goToMenu" /> <!-- ← TOPBAR TYLKO TUTAJ -->
      <EditorView @go-to-menu="goToMenu" />
    </template>
  </div>
</template>

<style>
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

/* ZMIANA: Grid 2-rzędowy gdy jest edytor */
.app-container {
  position: relative;
  min-height: 100vh;
  z-index: 1;
  display: grid;
  grid-template-rows: auto 1fr; /* TopBar auto + Editor 1fr */
}

/* Jak jest Launcher, to grid 1-rzędowy */
.app-container:has(.launcher) {
  grid-template-rows: 1fr;
}
</style>