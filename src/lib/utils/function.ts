import { invoke } from "@tauri-apps/api";

export async function copyToClipboard(value: string) {
    await invoke("copy_text", { value })
}