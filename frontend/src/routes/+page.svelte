<script>
    import { theme } from "$lib/stores/theme";
    import Mainmenucard from "$lib/mainmenucard.svelte";
</script>

<div class="p-6 flex items-center justify-center" data-theme={theme}>
    <div class="w-full max-w-xl">
        <Mainmenucard />
    </div>
</div>
