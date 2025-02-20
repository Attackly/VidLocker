<script lang="ts">
    import "../app.css";
    let { children } = $props();
    import Navbar from "$lib/navbar.svelte";
</script>

<Navbar />
<div class="global-bg-color min-h-screen">
    {@render children()}
</div>
