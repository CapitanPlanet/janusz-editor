<script setup lang="ts">
import { ref } from 'vue'
import { useFileDialog } from '@vueuse/core'
import { Howl } from 'howler'
import draggable from 'vuedraggable'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import Dropdown from 'primevue/dropdown'

// Typ dla jednej kwestii
interface Dialogue {
  id: number
  character: string
  text: string
  emotion: string
  voice: string | null
  duration: number
  background_override: string | null
}

const scene = ref({
  scene_id: 'day1_kuchnia',
  background: '',
  music: '',
  dialogues: [] as Dialogue[]
})

const characters = ['Janusz', 'Grażyna', 'Narrator']
const emotions = ['neutralna', 'wkurwiony', 'pijany', 'zadowolony']

const showAddDialog = ref(false)
const newLine = ref<Dialogue>({
  id: 0,
  character: 'Janusz',
  text: '',
  emotion: 'neutralna',
  voice: null,
  duration: 0,
  background_override: null
})

// Wybór pliku tła
const { open: openBg, onChange: onBgChange } = useFileDialog({
  accept: 'image/*',
  multiple: false
})

onBgChange((files) => {
  if (!files?.length) return
  // Tu w Tauri wywołamy command żeby skopiować plik do Assets/Backgrounds/
  const file = files[0]
  scene.value.background = file.name // Na razie nazwa, potem ścieżka z Rusta
})

// Wybór dźwięku dla kwestii
const { open: openVoice, onChange: onVoiceChange } = useFileDialog({
  accept: 'audio/*',
  multiple: false
})

onVoiceChange((files) => {
  if (!files?.length) return
  newLine.value.voice = files[0].name
})

// Preview dźwięku
function playSound(path: string) {
  const sound = new Howl({ src: [path] })
  sound.play()
}

function addDialogue() {
  newLine.value.id = Date.now()
  if (newLine.value.duration === 0) {
    newLine.value.duration = Math.max(1.5, newLine.value.text.length * 0.07) // auto
  }
  scene.value.dialogues.push({...newLine.value })
  showAddDialog.value = false
  newLine.value.text = ''
  newLine.value.voice = null
}

function saveScene() {
  // Tu invoke do Rusta: zapisz JSON
  console.log(JSON.stringify(scene.value, null, 2))
}
</script>

<template>
  <div class="editor">
    <div class="toolbar">
      <Button label="+ Dodaj kwestię" @click="showAddDialog = true" />
      <Button label="🖼️ Zmień tło" @click="openBg()" />
      <Button label="💾 Zapisz" @click="saveScene" severity="success" />
    </div>

    <div class="scene-settings">
      <p>Tło: {{ scene.background || 'brak' }}</p>
    </div>

    <draggable v-model="scene.dialogues" item-key="id" class="dialogue-list">
      <template #item="{ element }">
        <div class="dialogue-row">
          <strong>{{ element.character }}</strong>: {{ element.text }}
          <Button v-if="element.voice" icon="pi pi-play" text @click="playSound(element.voice)" />
          <span>{{ element.duration }}s</span>
        </div>
      </template>
    </draggable>

    <!-- Modal dodawania kwestii -->
    <Dialog v-model:visible="showAddDialog" header="Nowa kwestia" modal>
      <div class="form">
        <Dropdown v-model="newLine.character" :options="characters" placeholder="Postać" />
        <Textarea v-model="newLine.text" rows="3" placeholder="Co ma powiedzieć Janusz?" />
        <Dropdown v-model="newLine.emotion" :options="emotions" placeholder="Emocja" />
        <Button label="🎵 Dodaj dźwięk" @click="openVoice()" />
        <span v-if="newLine.voice">{{ newLine.voice }}</span>
        <InputNumber v-model="newLine.duration" placeholder="Czas trwania, 0=auto" />
      </div>
      <template #footer>
        <Button label="Anuluj" text @click="showAddDialog = false" />
        <Button label="Zapisz" @click="addDialogue" />
      </template>
    </Dialog>
  </div>
</template>