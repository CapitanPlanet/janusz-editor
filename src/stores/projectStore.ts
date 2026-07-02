import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { writeTextFile, readDir } from '@tauri-apps/plugin-fs'

interface Scene {
  Id: string
  Name?: string
  Background?: string
  [key: string]: any // na resztę pól Janusza
}

export const useProjectStore = defineStore('project', {
  state: () => ({
    currentProjectPath: null as string | null,
    projectData: null as any,
    currentSceneId: null as string | null,
    availableBackgrounds: [] as string[],
    saveStatus: ''
  }),
  
  getters: {
    currentScene: (state) => {
      if (!state.projectData) return null
    return state.projectData?.scenes.find((s: Scene) => s.Id === state.currentSceneId)
    },
    sceneList: (state) => state.projectData?.scenes || []
  },

 actions: {

  newProject() { console.log('Nowy projekt') },
  openProject() { console.log('Otwórz') },
  saveAsProject() { console.log('Zapisz jako') },
  buildGame() { console.log('Build') },

  async loadProject(path: string) {
    const json = await invoke('load_project', { path })
    this.projectData = JSON.parse(json as string)
    this.currentProjectPath = path
    this.currentSceneId = this.projectData.scenes[0]?.Id || null
    await this.scanBackgrounds()
  },

  async scanBackgrounds() {
    if (!this.currentProjectPath) return
    try {
      const bgPath = `${this.currentProjectPath}/assets/backgrounds`
      const entries = await readDir(bgPath)
      this.availableBackgrounds = entries
       .filter(e => e.name.endsWith('.jpg') || e.name.endsWith('.png'))
       .map(e => e.name.replace(/\.[^/.]+$/, ""))
      console.log('[STORE] Zeskanowano teł:', this.availableBackgrounds)
    } catch (e) {
      console.error('[STORE] Błąd skanowania teł:', e)
      this.availableBackgrounds = []
    }
  },

  async saveProject() {
    if (!this.currentProjectPath ||!this.projectData) return
    this.saveStatus = 'Zapisywanie...'
    const cleanData = JSON.parse(JSON.stringify(this.projectData))
    const json = JSON.stringify(cleanData, null, 2)
    const path = `${this.currentProjectPath}/project.json`
    await writeTextFile(path, json)
    this.saveStatus = 'Zapisano'
    setTimeout(() => { this.saveStatus = '' }, 2000)
  }, // <- DODANY PRZECINEK

  duplicateScene(sceneId: string) {
    if (!this.projectData) return
    const sceneToCopy = this.projectData.scenes.find((s: any) => s.Id === sceneId)
    if (!sceneToCopy) return

    const newScene = JSON.parse(JSON.stringify(sceneToCopy))
    newScene.Id = `${sceneToCopy.Id}_copy_${Date.now()}`
    newScene.SceneTitle = `${sceneToCopy.SceneTitle} - Kopia`

    const index = this.projectData.scenes.findIndex((s: any) => s.Id === sceneId)
    this.projectData.scenes.splice(index + 1, 0, newScene)

    this.currentSceneId = newScene.Id
  }
  
}
  
})