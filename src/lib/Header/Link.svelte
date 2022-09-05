<script lang="ts">
    export let icon: string;
    export let title: string;
    export let active: boolean;
    export let clickHandler: (e: MouseEvent) => void
</script>

<li class="group relative" on:click={clickHandler}>
    <img src={icon} alt={title} class="w-5" />
    <span
        class={`w-full h-0.5 bg-black absolute -bottom-1.5 rounded-2xl ${active ? "scale-100" : "scale-0"} group-hover:scale-100 transition-all`}
    />
</li>
