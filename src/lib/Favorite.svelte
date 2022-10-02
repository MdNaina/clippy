<script lang="ts">
    import { invoke, event } from "@tauri-apps/api";
    import Card from "./Favorite/Card.svelte";

    interface Starred {
        id: number,
        value: string
    }

    let list: Starred[] = [];

    async function fetchFavorite() {
        try {
            let newlist = await invoke("get_fav_list") as Starred[];
            console.log(list)
            list = newlist.reverse()
        }catch(e){
            console.log(e)
        }
    }
    event.listen("favorite-update", fetchFavorite);
    fetchFavorite()
</script>

<main class="w-ful flex-grow flex flex-col gap-1 p-2">
    {#if list.length == 0}
        <div class="h-full w-full flex justify-center items-center">
            <p>Favorite is Empty Now</p>
        </div>
    {:else}
        {#each list as item}
            <Card text={item.value} id={item.id} />
        {/each}
    {/if}
    <span class="min-h-[1px]" />
</main>
