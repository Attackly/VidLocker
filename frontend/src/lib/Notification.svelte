<!-- src/lib/components/Notifications.svelte -->
<script lang="ts">
    import { notifications } from "$lib/stores/notification";
    import { fly, fade } from "svelte/transition";

    $: $notifications;
</script>

<div class="fixed top-4 right-4 z-50 space-y-2">
    {#each $notifications as notif (notif.id)}
        <div
            in:fly={{ x: 200, duration: 200 }}
            out:fade={{ duration: 200 }}
            class="rounded-xl px-4 py-2 text-white shadow-lg max-w-sm
				{notif.type === 'error' && 'bg-error'}
				{notif.type === 'success' && 'bg-success'}
				{notif.type === 'info' && 'bg-info'}
				{notif.type === 'warning' && 'bg-warning text-black'}"
        >
            {notif.message}
        </div>
    {/each}
</div>
