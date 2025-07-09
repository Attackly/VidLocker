<script lang="ts">
    import { theme } from "$lib/stores/theme";
    import Linkcard from "$lib/Linkcard.svelte";
    import Thumbnailpreview from "$lib/Thumbnailpreview.svelte";
    import VideoDetails from "$lib/VideoDetails.svelte";
    import FileExplorer from "$lib/FileExplorer.svelte";
    import "../../app.css";
    import { link } from "$lib/stores/linkstore";
    import { send } from "$lib/stores/notification";

    async function download() {
        if ($link.length < 28) {
            // TODO make valid checks better
            send("The link is to short to be valid.", "warning");
            return;
        }

        const res = await fetch("/api/downloadVideo", {
            method: "POST",
            body: JSON.stringify({ url: $link }),
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!res.ok) {
            console.log(
                `There was an error downloading the Video: ${res.body}`,
            );
            send(
                `There was an error downloading the Video: ${res.body}`,
                "error",
            );
        } else {
            send("Video has been queued for download", "success");
            return;
        }
    }
</script>

<div class="flex flex-col items-center" data-theme={theme}>
    <Linkcard />
    <Thumbnailpreview />
    <VideoDetails />
    <FileExplorer />

    <button
        on:click={download}
        type="button"
        class="button-bg sm:w-3/4 lg:w-1/2 mb-30 rounded text-primary py-2"
    >
        Download
    </button>
</div>
