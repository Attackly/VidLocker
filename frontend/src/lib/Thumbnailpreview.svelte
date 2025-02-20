<script lang="ts">
    import { link } from "$lib/stores/linkstore"; // Adjust this import as needed
    let viewkey: string = "";
    // Function to run when the link store changes

    function onLinkChange(newLink: string) {
        if (viewkey.length != 11) {
            viewkey = newLink.split("v=")[1];
        } else if (viewkey.length == 11) {
            viewkey = newLink;
        }

        console.log("Viewkey in the Thumbniail:", viewkey);
    }

    // Subscribe to the store
    $: {
        $link; // Automatically tracks the store
        onLinkChange($link); // Calls the function whenever the store changes
    }
</script>

<div
    class="w-2/3 mt-8 p-4 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
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
                class="absolute inset-0 flex items-center justify-center bg-gray-200 text-center"
            >
                <span class="text-gray-800 text-xl font-semibold"
                    >Placeholder</span
                >
            </div>
        {/if}
    </div>
</div>
