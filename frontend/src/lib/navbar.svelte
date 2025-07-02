<script>
    import { theme } from "$lib/stores/theme";
    import Notifications from '$lib/Notification.svelte';
    
    let isOpen = $state(false);

    function toggleTheme() {
        theme.update((t) => (t === "bird" ? "death-dark" : "bird"));
    }
</script>

<Notifications />

<nav class="bg-gray-900 text-white px-6 py-4">
    <div class="flex justify-between items-center max-w-6xl mx-auto">
        <a href="/" class="text-xl font-bold">VidLocker</a>

        <div class="hidden md:flex space-x-6">
            <a href="/" class="hover:text-gray-400">Home</a>
            <a href="/Download" class="hover:text-gray-400">Download</a>
            <a href="/upload" class="hover:text-gray-400">Upload</a>
            <a href="/settings" class="hover:text-gray-400">Settings</a>
            <a href="/Shelf" class="hover:text-gray-400">Shelf</a>
        </div>

        <div class="flex items-center space-x-4">
            <button onclick={toggleTheme} class="p-2 bg-gray-700 rounded">
                <span>{$theme === "bird" ? "🐦‍⬛" : "🪦"}</span>
            </button>
            <button class="md:hidden p-2" onclick={() => (isOpen = !isOpen)}>
                {isOpen ? "✖" : "☰"}
            </button>
        </div>
    </div>
</nav>

{#if isOpen}
    <div class="md:hidden bg-gray-800 text-white px-6 py-4 space-y-2">
        <a href="/" class="block hover:text-gray-400">Home</a>
        <a href="/Download" class="block hover:text-gray-400">Download</a>
        <a href="/upload" class="block hover:text-gray-400">Upload</a>
        <a href="/settings" class="block hover:text-gray-400">Settings</a>
        <a href="/Shelf" class="block hover:text-gray-400">Shelf</a>
    </div>
{/if}
