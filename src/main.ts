import { invoke } from '@tauri-apps/api/core'
import './style.css'

const status = document.querySelector<HTMLElement>('#status')

async function connect(): Promise<void> {
  try {
    const url = await invoke<string>('harness_url')
    if (url === window.location.href) return
    window.location.replace(url)
  } catch (error) {
    if (status) {
      status.textContent = `DeepSeek Harness 启动失败：${String(error)}`
      status.classList.add('error')
    }
  }
}

void connect()
