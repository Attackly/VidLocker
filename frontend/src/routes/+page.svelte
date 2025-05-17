<script>
    import { theme } from "$lib/stores/theme";
    import Mainmenucard from "$lib/mainmenucard.svelte";
</script>

<div class="p-6 flex items-center justify-center" data-theme={theme}>
    <Mainmenucard />
</div>
