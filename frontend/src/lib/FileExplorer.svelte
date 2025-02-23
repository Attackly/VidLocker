<script lang="ts">
    import { onMount } from "svelte";

    let isOpen = false;
    let data: any[] = [];
    let currentDir = "/";

    async function get_Files() {
        // fetch files from server.
        //
        try {
            const res = await fetch(
                `/api/files?dir=${encodeURIComponent(currentDir)}`,
            );
            if (!res.ok)
                throw new Error("Request failed with Status: ${res.status}");

            const raw_data = await res.json();

            data = raw_data.files;
        } catch (error) {
            console.error("Error Fetching or parsing response:", error);
        }

        function navigateTo(dir: string) {
            currentDir = dir;
            get_Files();
        }

        onMount(get_Files);
    }
</script>

<div
    class="sm:w-3/4 mt-5 p-3 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
    <div class="card-header">
        <button on:click={() => (isOpen = !isOpen)}>
            {#if isOpen}
                🔼
            {:else}
                🔽
            {/if}
        </button>
    </div>
    {#if isOpen}
        {#if data.length > 0}
            <table class="w-full">
                <thead class="">
                    <tr>
                        <th class="px-4 py-2 text-left">Name</th>
                        <th class="px-4 py-2 text-left">Size</th>
                        <th class="px-4 py-2 text-left">Action</th>
                    </tr>
                </thead>
                <tbody>
                    {#each data as row}
                        <tr>
                            <td class="px-4 py-2">{row.Name}</td>
                            <td class="px-4 py-2">{row.Size}</td>
                            <td>
                                <button
                                    class="bg-red-700 p-1 rounded"
                                    id="delete"
                                    data-id={row.id}
                                >
                                    <svg
                                        class="w-5 h-5"
                                        fill="#ffffff"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 448 512"
                                        ><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path
                                            d="M135.2 17.7L128 32 32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0-7.2-14.3C307.4 6.8 296.3 0 284.2 0L163.8 0c-12.1 0-23.2 6.8-28.6 17.7zM416 128L32 128 53.2 467c1.6 25.3 22.6 45 47.9 45l245.8 0c25.3 0 46.3-19.7 47.9-45L416 128z"
                                        />
                                    </svg>
                                </button>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {:else}
            <div
                class="p-4 text-sm text-yellow-800 rounded-lg bg-yellow-50 dark:bg-gray-800 dark:text-yellow-300"
                role="alert"
            >
                Nothing to display yet. Better Start Downloading
            </div>
        {/if}
    {/if}
</div>
{#if isOpen}
    <div class="lg:w-1/3 mt-8 card p-4"></div>
{/if}
