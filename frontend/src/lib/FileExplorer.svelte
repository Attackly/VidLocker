<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  const illegalChars = "/\0";

  let isOpen = $state(false);
  let currentDir = $state("/");
  let newFolderName = $state("");
  let files = $state({
    loading: true,
    data: [] as any[],
    error: null as string | null,
  });
  let version = $state(0);
  let currentDirArray = $state(["/"]);

  let breadcrumbs = $derived(() => {
    const parts = currentDir.split("/").filter((p) => p);
    let path = "";
    return parts.map((part) => {
      path += `/${part}`;
      return { name: part, path };
    });
  });

  function formatBytes(bytes: number) {
    if (bytes === 0) return "0 Bytes";
    const units = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / 1024 ** i).toFixed(2)} ${units[i]}`;
  }

  $effect(() => {
    async function getFiles() {
      files.loading = true;
      files.error = null;
      try {
        const res = await fetch(
          `/api/files?dir=${encodeURIComponent(currentDir)}`
        );
        if (!res.ok)
          throw new Error(`Request failed with status: ${res.status}`);

        const raw_data = await res.json();
        if (!Array.isArray(raw_data))
          throw new Error("API did not return an array.");

        files.data = raw_data;
      } catch (e: any) {
        files.error = e.message;
        console.error("Error fetching or parsing response:", e);
      } finally {
        files.loading = false;
      }
    }

    getFiles();
  });

  function navigateTo(path: string) {
    currentDir = path;
  }

  async function deleteItem(name: string) {
    const fullPath = `${currentDir}/${name}`.replace("//", "/");
    try {
      const res = await fetch(`/api/files/${encodeURIComponent(fullPath)}`, {
        method: "DELETE",
      });
      if (!res.ok) throw new Error("Failed to delete file");
      version++;
    } catch (e) {
      console.error(`Failed to delete item: ${name}`, e);
    }
  }

  async function createFolder() {
    if (!newFolderName) return;
    const fullPath = `${currentDir}/${newFolderName}`.replace("//", "/");
    try {
      const res = await fetch("/api/folders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ path: fullPath }),
      });
      if (!res.ok) throw new Error("Failed to create folder");
      newFolderName = "";
      version++;
    } catch (e) {
      console.error(`Failed to create folder: ${newFolderName}`, e);
    }
  }
</script>

<div
  class="sm:w-3/4 lg:w-1/2 mt-5 p-3 mb-5 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
  <div class="card-header">
    <button onclick={() => (isOpen = !isOpen)} class="w-full text-left">
      {#if isOpen}
        🔼 Close File Explorer
      {:else}
        🔽 Open File Explorer
      {/if}
    </button>
  </div>
  {#if isOpen}
    <ol
      class="flex w-full flex-wrap items-center rounded-md px-4 py-2 text-primary text-lg"
    >
      <li
        class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg"
      >
        <button type="button" onclick={() => navigateTo("/")}> / </button>
      </li>
      {#each currentDirArray as dir}
        {#if dir != "" && dir != null && dir != undefined && dir != "/"}
          <li
            class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg"
          >
            <span class="pointer-events-none mx-2 text-primary text-lg">
              >
            </span>
            <button type="button" onclick={() => navigateTo(dir)}>
              {dir}
            </button>
          </li>
        {/if}
      {/each}
    </ol>
    {#if files.loading}
      <div class="p-4 text-center">Loading...</div>
    {:else if files.error}
      <div class="p-4 text-sm text-red-800 rounded-lg bg-red-50" role="alert">
        Error: {files.error}
      </div>

      <!-- TODO CHANGE THIS BACK --->
    {:else if files.data.length > 0}
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="px-2 py-2 text-left">Type</th>
            <th class="px-4 py-2 text-left">Name</th>
            <th class="px-4 py-2 text-left">Size</th>
            <th class="px-4 py-2 text-left">Action</th>
          </tr>
        </thead>
        <tbody>
          <!-- TODO CHANGE THIS BACK --->
          {#each files.data as row}
            <tr class="row-bg-1 odd:row-bg-2">
              {#if row.is_directory == false}
                <td class="px-2 py-2">
                  <svg
                    class="svg-primary"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512"
                    ><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.-->
                    <path
                      d="M0 96C0 60.7 28.7 32 64 32l384 0c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 480c-35.3 0-64-28.7-64-64L0 96zM48 368l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm368-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM48 240l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0c-8.8 0-16 7.2-16 16zm368-16c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM48 112l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16L64 96c-8.8 0-16 7.2-16 16zM416 96c-8.8 0-16 7.2-16 16l0 32c0 8.8 7.2 16 16 16l32 0c8.8 0 16-7.2 16-16l0-32c0-8.8-7.2-16-16-16l-32 0zM160 128l0 64c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-64c0-17.7-14.3-32-32-32L192 96c-17.7 0-32 14.3-32 32zm32 160c-17.7 0-32 14.3-32 32l0 64c0 17.7 14.3 32 32 32l128 0c17.7 0 32-14.3 32-32l0-64c0-17.7-14.3-32-32-32l-128 0z"
                    />
                  </svg>
                </td>
                <td class="px-4 py-2">{row.name}</td>
                <td class="px-4 py-2">
                  {formatBytes(row.file_size)}
                </td>
              {:else}
                <td class="py-2 px-2">
                  <svg
                    class="svg-primary"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 512 512"
                    ><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path
                      d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"
                    />
                  </svg>
                </td>
                <td>
                  <button
                    class="rounded-full py-2 px-4 cursor-pointer"
                    type="button"
                    onclick={() => navigateTo(row.name)}
                  >
                    {row.name}
                  </button>
                </td>
                <td class="px-4 py-2">{formatBytes(row.file_size - 16)}</td>
              {/if}
              <td class="text-center">
                <button
                  class="bg-red-700 p-1 rounded cursor-pointer items-center"
                  id="delete"
                  data-id={row.name}
                  onclick={() => deleteItem(row.name)}
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

      <div class="pt-4">
        <label for="newFolderName">Folder Name</label>
        <div class="flex w-full">
          <input
            class="w-2/3 border rounded"
            id="newFolderName"
            bind:value={newFolderName}
          />
          <button class="w-1/3 button-bg"> Create new Folder </button>
        </div>
      </div>
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
