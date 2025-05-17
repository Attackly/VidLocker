<script lang="ts">
    import "../app.css";
    import Navbar from "$lib/navbar.svelte";
    import Notification from "$lib/Notification.svelte";

    let { children } = $props();
</script>

<Navbar />

<Notification />

<div class="global-bg-color min-h-screen">
    {@render children()}
</div>
