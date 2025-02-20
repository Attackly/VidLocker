<script lang="ts">
    import { theme } from "$lib/stores/theme";
    import Linkcard from "$lib/Linkcard.svelte";
    import Thumbnailpreview from "$lib/Thumbnailpreview.svelte";
</script>

<div class="flex flex-col items-center" data-theme={theme}>
    <Linkcard />

    <Thumbnailpreview />
</div>
