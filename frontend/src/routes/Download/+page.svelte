<script lang="ts">
    import { theme } from "$lib/stores/theme";
    import Linkcard from "$lib/Linkcard.svelte";
    import Thumbnailpreview from "$lib/Thumbnailpreview.svelte";
    import VideoDetails from "$lib/VideoDetails.svelte";
    import FileExplorer from "$lib/FileExplorer.svelte";
    import "../../app.css";
    import { link } from "$lib/stores/linkstore";
</script>

<p>Current link {link}</p>
<div class="flex flex-col items-center" data-theme={theme}>
    <Linkcard />
    <Thumbnailpreview />
    <VideoDetails />
    <FileExplorer />

    <button type="button" class="bg-primary w-1/2 mb-30 border rounded">
        Download
    </button>t
</div>
