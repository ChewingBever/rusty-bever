import { createApp } from 'vue'
import App from './App.vue'
// @ts-ignore
import { makeServer } from "./server"

if (process.env.NODE_ENV === "development") {
    makeServer()
}

createApp(App).mount('#app')
