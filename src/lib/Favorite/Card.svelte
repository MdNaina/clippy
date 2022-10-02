<script lang="ts">
    import { clipboard, event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/tauri";
    import optionIcon from "../../assets/options.svg";

    export let text: string;
    export let id: number;

    let optionStatus = false;

    const removeFromFavorite = async(e: MouseEvent) => {
        await invoke("remove_from_favorite", { id })
        await event.emit("favorite-update");
        toggleOptionOff(e)
    };

    async function copyToClipboard() {
        let res = await clipboard.writeText(text);
        console.log(res);
    }

    function toggleOptionOn(e: MouseEvent) {
        optionStatus = !optionStatus;
    }

    function toggleOptionOff(e: MouseEvent) {
        optionStatus = false;
    }

</script>

<div
    class="relative h-[5rem] w-full mb-1 bg-gray-300 text-sm p-2 rounded-md cursor-pointer"
    on:click|stopPropagation={copyToClipboard}
>
    <p>{text.slice(0, 130)}</p>
    <div class="p-2 px-3 absolute top-4 right-2" on:click|stopPropagation={toggleOptionOn}>
        <img
            src={optionIcon}
            alt="option"
            class="w-1.5 "
        />
    </div>

    <div
        class={`absolute right-2 top-4 w-max h-min bg-transparent pt-0.5 pr-4 scale-y-0 ${
            optionStatus ? "scale-y-100" : ""
        } `}
        on:mouseleave={toggleOptionOff}
        tabindex="0"
    >
        <div
            class="w-28 h-12 flex flex-col items-start justify-center bg-white p-1 rounded-md"
        >
            <button
                on:click|stopPropagation={removeFromFavorite}
                class="w-full whitespace-nowrap text-start ">Remove</button
            >
        </div>
    </div>
</div>
