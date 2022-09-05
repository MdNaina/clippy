<script lang="ts">
    import { invoke, event } from "@tauri-apps/api"
import Card from "./Clipboard/Card.svelte"

    let list: string[] = []

    async function fetchClipList() {
        let newList: string[] = await invoke("get_clip_list")
        console.log(newList)
        list = newList.reverse()
    }

    event.listen("clipboard-update", fetchClipList)

    fetchClipList()
</script>

<main class="w-ful flex-grow flex flex-col gap-1 p-2">
    {#each list as text}
        <Card {text} />
    {/each}
    <span class="min-h-[1px]" />
</main>