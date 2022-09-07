<script lang="ts">
    import { clipboard } from "@tauri-apps/api"

    export let text: string;

    async function copyToClipboard() {
        let res = await clipboard.writeText(text)
        console.log(res)
    }
</script>

<div class="h-[5rem] w-full mb-1 bg-gray-300 text-sm p-2 rounded-md cursor-pointer" on:click={copyToClipboard}>
    <p>{text.slice(0, 130)}</p>
</div>
