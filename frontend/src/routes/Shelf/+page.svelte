<script lang="ts">
    // TODO Refactor this shit. It looks ugly and shitty.
    const illegalChars = "\0";
    const units = ["Bytes", "KB", "MB", "GB"];

    import "../../app.css";
    import Modal from "$lib/modal.svelte"

    let showModal = false;
    let FileName = "";
    let data: any[] = []
    let currentDir = "/";
    let currentDirArray = ["/"];
    let newFolderName = "";
    let filePromise = get_Files(currentDir);
    $: filePromise = get_Files(currentDir);

    function formatBytes(bytes: number) {
        if (bytes === 0) return "0 Bytes";

        const i = Math.floor(Math.log(bytes) / Math.log(1000));
        return (bytes / Math.pow(1000, i)).toFixed(2) + " " + units[i];
    }

    async function cd(dir: string) {
        if (dir === "/") {
            currentDir = "/";
            currentDirArray = ["/"];
        } else {
            if (currentDir === "/") {
                currentDir = `/${dir}`;
            } else {
                currentDir = `${currentDir}/${dir}`;
            }
            currentDirArray = currentDir.split("/").filter(Boolean);
        }
    }
    async function deleteFile(path: string) {
        const res = await fetch(`/api/file${path.substring(1)}`, {
            method: "DELETE",
        });
        if (!res.ok) {
            console.error(`Failed to delete file with id: ${path}`);
            return null;
        }

        filePromise = get_Files(currentDir);
        return path;
    }

    async function get_Files(dir: string) {
        try {
            const res = await fetch(`/api/files?dir=${encodeURIComponent(dir)}`);

            if (!res.ok) {
                console.error(`Request failed with status: ${res.status}`);
            }

            const raw_data = await res.json();
            console.log("First file:", raw_data[0]);

            if (!Array.isArray(raw_data)) {
                console.error("Expected an array, got:", raw_data);
                return [];
            }

            return raw_data;

        } catch (error) {
            console.error("Error fetching files from the Server:", error);
            return [];
        }


    }

    async function downloadFile(filename: string) {
        try {
            let filenameURI = encodeURIComponent(filename);
            console.log("Downloading file:", filenameURI);
            const res = await fetch(`/api/files/download?filename=${filenameURI}`);

            const blob = await res.blob();
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = `${filename}`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);

            URL.revokeObjectURL(url); // Clean up
        } catch (error) {
            console.error("Error downloading file:", error);
        }
    }

    async function create_new_dir(newDirName : string) {
        // TODO
        if(newDirName.includes(illegalChars)) {
            newFolderName = "";
            return 1;
        }

        return 0;
    }
    const handleDeleteClick = (filename: string) => {
        FileName = filename;
        showModal = true;
    };

    const confirmDelete = () => {
        deleteFile(FileName);
        showModal = false;
        FileName = "";
    };

    const cancelDelete = () => {
        showModal = false;
        FileName = "";
    };

</script>

<div
        class="sm:w-3/4 lg:w-1/2 mt-5 p-3 mb-5 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative justify-center flex mx-auto"
>
    <div class="flex flex-col max-width">
        <ol
                class="flex w-full flex-wrap items-center rounded-md px-4 py-2 text-primary text-lg"
        >
            <li
                    class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg"
            >
                <button type="button" on:click={() => cd("/")}> /</button>
            </li>
            {#each currentDirArray as dir}
                {#if dir !== "" && dir != null && dir !== undefined && dir !== "/"}
                    <li
                            class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg"
                    >
            <span class="pointer-events-none mx-2 text-primary text-lg">
              >
            </span>
                        <button type="button" on:click={() => cd(dir)}>
                            {dir}
                        </button>
                    </li>
                {/if}
            {/each}
        </ol>

        <div class="pb-10"></div>
        {#await filePromise then data}
        <table class="max-width table-fixed w-full">
            <thead class="border-b">
            <tr>
                <th class="w-[5%] px-4 py-2 text-left">Type</th>
                <th class="w-[70%] px-4 py-2">Name</th>
                <th class="w-[15%] px-4 py-2">Size</th>
                <th class="w-[10%] px-4 py-2">Action</th>
            </tr>
            </thead>
            <tbody>
            <!-- TODO CHANGE THIS BACK --->
            {#each data as row}
                <tr class="row-bg-1 odd:row-bg-2">
                    {#if row.is_directory === false}
                        <td class="px-2 py-2"
                        >
                            <svg
                                    class="svg-primary w-6 h-6"
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 512 512"
                            >
                                <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                <path
                                        d="M0 96C0 60.7 28.7 32 64 32l384 0c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96zM48 368l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm368-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM48 240l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm368-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM48 112l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16L64 96c-8.8 0-16 7.2-16 16zM416 96c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM160 128l0 64c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-64c0-17.7-14.3-32-32-32L192 96c-17.7 0-32 14.3-32 32zm32 160c-17.7 0-32 14.3-32 32l0 64c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-64c0-17.7-14.3-32-32-32l-128 0z"
                                />
                            </svg
                            >
                        </td
                        >
                    {:else}
                        <td class="py-2 px-2">
                            <svg
                                    class="svg-primary w-6 h-6"
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 512 512"
                            >
                                <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                <path
                                        d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"
                                />
                            </svg>
                        </td>
                    {/if}
                    {#if row.is_directory === false}
                        <td class="px-4 py-2"> {row.name}</td>
                    {:else}
                        <td>
                            <button
                                    class="rounded-full py-2 px-4 cursor-pointer"
                                    type="button"
                                    on:click={() => cd(row.name)}
                            >
                                {row.name}
                            </button>
                        </td>
                    {/if}

                    <td class="px-4 py-2">{formatBytes(row.file_size)}</td>
                    <td class="text-center">
                        <div>
                            <button
                                    class="bg-red-700 p-1 rounded cursor-pointer items-center"
                                    id="delete"
                                    data-id={row.path}
                                    on:click={() => handleDeleteClick(row.path)}
                            >
                                <svg
                                        class="w-5 h-5"
                                        fill="#ffffff"
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 448 512"
                                >
                                    <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                    <path
                                            d="M135.2 17.7L128 32 32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0-7.2-14.3C307.4 6.8 296.3 0 284.2 0L163.8 0c-12.1 0-23.2 6.8-28.6 17.7zM416 128L32 128 53.2 467c1.6 25.3 22.6 45 47.9 45l245.8 0c25.3 0 46.3-19.7 47.9-45L416 128z"
                                    />
                                </svg>
                            </button>

                            {#if row.is_directory === false}
                                <button
                                        class="rounded p-1 cursor-pointer items-center bg-blue-800"
                                        type="button"
                                        on:click={() => downloadFile(row.name)}
                                >
                                    <svg
                                            class="w-5 h-5"
                                            fill="#ffffff"
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 512 512"
                                    >
                                        <!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                                        <path
                                                d="M288 32c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 242.7-73.4-73.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l128 128c12.5 12.5 32.8 12.5 45.3 0l128-128c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L288 274.7 288 32zM64 352c-35.3 0-64 28.7-64 64l0 32c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-32c0-35.3-28.7-64-64-64l-101.5 0-45.3 45.3c-25 25-65.5 25-90.5 0L165.5 352 64 352zm368 56a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"
                                        />
                                    </svg
                                    >
                                </button>
                            {/if}
                        </div>
                    </td>
                </tr>
            {/each}
            </tbody>
        </table>
        {/await}
        <div class="pt-4">
            <label for="newFolderName">Folder Name</label>
            <div class="flex w-full">
                <input
                        class="w-2/3 border rounded"
                        bind:value={newFolderName}
                />
                <button class="w-1/3 button-bg cursor-pointer" type="button" on:click={() => create_new_dir(newFolderName)}>Create new Folder</button>
            </div>
        </div>
    </div>
</div>

<Modal
        open={showModal}
        onConfirm={confirmDelete}
        onCancel={cancelDelete}
/>