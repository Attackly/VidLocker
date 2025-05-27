<script lang="ts">
    import { link } from "$lib/stores/linkstore"; // Adjust this import as needed
    let viewkey: string = "";
    // Function to run when the link store changes

    function onLinkChange(newLink: string) {
        console.log("Link has changed in the Previewe");
        if (newLink.length != 11) {
            viewkey = newLink.split("?v=")[1];
        } else if (newLink.length == 11) {
            viewkey = newLink;
        }

        console.log("Viewkey in the Thumbniail:", viewkey);
    }

    link.subscribe((value) => {
        if (value === undefined) {
            console.log("Link is undefined");
        } else {
            onLinkChange(value);
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
