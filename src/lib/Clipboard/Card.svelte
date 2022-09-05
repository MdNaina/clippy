<script lang="ts">
    import { invoke } from "@tauri-apps/api"

    export let text: string;

    async function copyToClipboard() {
        await invoke("set_into_clipboard", {value: text})
    }
</script>

<div class="h-[5rem] w-full mb-1 bg-gray-300 text-sm p-2 rounded-md cursor-pointer" on:click={copyToClipboard}>
    <p>{text.slice(0, 130)}</p>
</div>
