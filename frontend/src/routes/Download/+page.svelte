<script lang="ts">
    import { theme } from "$lib/stores/theme";
    import Linkcard from "$lib/Linkcard.svelte";
    import Thumbnailpreview from "$lib/Thumbnailpreview.svelte";
    import VideoDetails from "$lib/VideoDetails.svelte";
    import FileExplorer from "$lib/FileExplorer.svelte";
</script>

<div class="flex flex-col items-center" data-theme={theme}>
    <Linkcard />
    <Thumbnailpreview />
    <VideoDetails />
    <FileExplorer />
</div>
