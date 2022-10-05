<script lang="ts">
import Clipboard from "./lib/Clipboard.svelte";
import Header from "./lib/Header.svelte";
import Favorite from "./lib/Favorite.svelte";


let currentTab = "clipboard"

let changeTab = (link: string) => {
  currentTab = link
}

</script>

<main class="container  bg-gray-400">
  <div class="h-[4rem]">
    <Header {currentTab} {changeTab} />
  </div>
  <div class="h-[calc(100vh-4rem)] overflow-y-scroll">
    {#if currentTab == "clipboard"}
    <Clipboard />
    {:else if currentTab == "favorite"}
    <Favorite />
    {:else}
    <div>Comming Soon</div>
    {/if}
  </div>
</main>
