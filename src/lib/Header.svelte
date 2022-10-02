<script lang="ts">
    import clipIcon from "../assets/clipboard.png";
    import starIcon from "../assets/star.png";
    import keyIcon from "../assets/key.png";
    import settingIcon from "../assets/settings.png";
    import Link from "./Header/Link.svelte";

    const routes = [
        {
            title: "Clipboard",
            icon: clipIcon,
            link: "clipboard",
        },
        {
            title: "Favorite",
            icon: starIcon,
            link: "favorite",
        },
        {
            title: "Keys",
            icon: keyIcon,
            link: "key",
        },
        {
            title: "Settings",
            icon: settingIcon,
            link: "settings",
        },
    ];

    export let currentTab: string;
    export let changeTab: (link:string) => void

    let clickHandler = (link: string) => (e: MouseEvent) => {
        changeTab(link)
    }

</script>

<div class="h-full flex flex-col justify-between">
    <h1 class="p-1.5">{routes.find(({link}) => link ==currentTab).title}</h1>
    <ul class="flex justify-around pb-2">
        {#each routes as { title, link, icon }}
            <Link {title} {icon} active={currentTab == link} clickHandler={clickHandler(link)} />
        {/each}
    </ul>
</div>
