import { createApp } from "vue";
import { createPinia } from "pinia"; // 1. dodaj import
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia(); // 2. stwórz pinia
app.use(pinia); // 3. podłącz do app
app.mount("#app");