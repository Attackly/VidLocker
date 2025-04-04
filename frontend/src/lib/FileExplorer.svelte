<script lang="ts">
    import { onMount } from "svelte";
    let exampledata = [
        {
            Name: "example",
            Size: "100000",
            file: "mp3",
        },
        {
            Name: "example2",
            Size: "200000",
            file: "mp4",
        },
        {
            Name: "example3",
            Size: "300000",
            file: "folder",
        },
    ];
    let isOpen = false;
    let data: any[] = [];
    let currentDir = "/";

    function cd(dir: string) {
        currentDir = dir;
        get_Files(dir);
    }

    async function deleteFile(id: string) {
        const res = await fetch(`/api/files/${id}`, {
            method: "DELETE",
        });
        if (!res.ok) {
            console.error(`Failed to delete file with id: ${id}`);
            return null;
        }
        return id;
        console.log(`Printing thing with id: ${id}`);
    }

    async function get_Files(dir: string) {
        try {
            const res = await fetch(
                `/api/files?dir=${encodeURIComponent(dir)}`,
            );
            if (!res.ok) {
                console.log("Message ${res.message}");
                throw new Error("Request failed with Status: ${res.status}");
            }
            const raw_data = await res.json();

            return raw_data.files;
        } catch (error) {
            console.error("Error Fetching or parsing response:", error);
            return null;
        }
    }
</script>

<div
    class="sm:w-3/4 lg:w-1/2 mt-5 p-3 mb-50 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
    <div class="card-header">
        <button on:click={() => (isOpen = !isOpen)} class="w-full text-left">
            {#if isOpen}
                🔼 Close File Explorer
            {:else}
                🔽 Open File Explorer
            {/if}
        </button>
    </div>
    {#if isOpen}
        {#await get_Files(currentDir) then data}
            <!-- TODO CHANGE THIS BACK --->
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
                        <!-- TODO CHANGE THIS BACK --->
                        {#each data as row}
                            <tr>
                                {#if row.file !== "folder"}
                                    <td class="px-4 py-2"> {row.Name}</td>
                                {:else}
                                    <td>
                                        <button
                                            class="rounded-full py-2 px-4 cursor-pointer"
                                            type="button"
                                            on:click={() => cd(row.Name)}
                                        >
                                            {row.Name}
                                        </button>
                                    </td>
                                {/if}
                                <td class="px-4 py-2">{row.Size}</td>
                                <td>
                                    <button
                                        class="bg-red-700 p-1 rounded cursor-pointer"
                                        id="delete"
                                        data-id={row.Name}
                                        on:click={() => deleteFile(row.Name)}
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
        {/await}
    {/if}
</div>
{#if isOpen}
    <div class="lg:w-1/3 mt-8 card p-4"></div>
{/if}
