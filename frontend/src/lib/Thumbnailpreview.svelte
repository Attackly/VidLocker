<script lang="ts">
    import { link } from "$lib/stores/linkstore";

    let viewkey = $derived(() => {
        const newLink = $link

        if (!newLink) {
            return "";
        }

        if (newLink.includes("?v=")) {
            return newLink.split("?v=")[1];
        }

        else if (newLink.length === 11) {
            return newLink;
        }

        return "";
    });

    $effect(() => {
        if (viewkey) {
            console.log("Viewkey in the Thumbnail:", viewkey);
        }
    });
</script>

<div
    class="sm:w-3/4 lg:w-1/2 mt-5 p-3 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
    <div class="w-full h-0 pb-[56.25%] relative">
        {#if viewkey}
            <img
                src={`https://img.youtube.com/vi/${viewkey}/hqdefault.jpg`}
                alt="YouTube Thumbnail"
                class="absolute inset-0 w-full h-full object-cover"
            />
        {:else}
            <div
                class="absolute inset-0 flex items-center justify-center bg-gray-200 text-center rounded-lg"
            >
                <span class="text-gray-800 text-xl font-semibold"
                    >Insert a link above</span
                >
            </div>
        {/if}
    </div>
</div>
