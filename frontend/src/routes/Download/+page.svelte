<script lang="ts">
    interface FileItem {
        Name: string;
        Size: number;
        ID: string;
        IsFolder: boolean;
    }

    import { onMount } from "svelte";

    let error_message: string = "";
    let validation: string = "";
    let url: string = ""; // YouTube video URL
    let viewkey: string = ""; // YouTube video ID
    let videoTitle: string = ""; // Video title from YouTube API
    let useApi: boolean = true; // Flag to determine if the YouTube Data API should be used
    let error: string = ""; // Error message
    let isError: boolean = false; // Flag to determine if an error occurred
    let video: any = null; // Video object from YouTube API
    let video_published_at: Date | null = null;
    let video_published_str: string = "";
    let disabled = true;
    let isOpen = true;
    let data: any[] = [
        {
            Name: "Baerys newyear karaoke",
            Size: 1000,
            ID: "abc123-321cba",
            IsFolder: true,
        },
    ];
    function url_verify() {
        // ViewKey
        if (url.length == 11) {
            validation = "input-success";
        }
        // full link with HTTPS or without HTTPS
        else if (url.length == 43 || url.length == 35) {
            validation = "input-success";
        }
        // Shortend Youtube link
        else if (url.length == 28 || url.length == 20) {
            validation = "input-success";
        }
        // Between Viewkey and link
        else if (url.length < 23 && url.length > 11) {
            validation = "input-error";
            disabled = true;
            return;
        }
        // Not even a viewkey
        else if (url.length < 11) {
            validation = "input-error";
            disabled = true;
            return;
        }

        let url_split_array = url.split("v=");

        console.log(url_split_array[1].toString());
        viewkey = url_split_array[1].toString();
        getYouTubeTitle();
        disabled = false;
        console.log("End");
    }

    // Function to check if API key is available
    async function checkApiKey() {
        try {
            const response = await fetch("/api/yt/mode");
            const data = await response.json();
            console.log(data);
            getYouTubeTitle(); // Start fetching the video title if API key exists
        } catch (err) {
            error = "Failed to check API key";
            console.error(err);
        }
    }

    // Function to fetch the video title using the YouTube Data API
    async function getYouTubeTitle() {
        if (!viewkey || !useApi) return;

        const response = await fetch("/api/yt/getTitle", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: '{"url": "' + viewkey + '"}',
        });
        const data = await response.json();

        if (data.video == null) {
            isError = true;
            disabled = true;
        } else {
            isError = false;
        }
        video = data.video;
        videoTitle = data.video.title;
        video_published_at = new Date(data.video.published_at);

        video_published_str = video_published_at.toLocaleString();
    }

    async function download_video() {
        if (!viewkey) return;

        const response = await fetch("/api/downloadVideo", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: '{"url": "' + viewkey + '"}',
        });
        const data = await response.json();
        console.log(data);
        if (data.status == 200) {
            console.log("Downloaded");
        } else {
            console.log("Failed");
            error_message = data.message;
        }
    }

    // Automatically run when the component mounts
    onMount(() => {
        checkApiKey(); // Check API key availability when the page loads
    });
</script>

<div class="flex flex-col items-center justify-center">
    <div class="lg:w-1/3 mt-8 card p-4">
        <label class="label">
            <span>Url</span>
            <input
                class="input {validation}"
                type="text"
                placeholder="https://www.youtube.com/watch?v=dQw4w9WgXcQ"
                bind:value={url}
                on:input={url_verify}
            />
        </label>
    </div>

    <div class="lg:w-1/3 mt-5 card p-4">
        <div
            class="relative w-full bg-gray-400 rounded shadow-lg"
            style="padding-top: 56.25%;"
        >
            {#if viewkey}
                <img
                    src={`https://img.youtube.com/vi/${viewkey}/hqdefault.jpg`}
                    alt="YouTube Thumbnail"
                    class="absolute inset-0 w-full h-full object-cover"
                />
            {:else}
                <div class="absolute inset-0 flex items-center justify-center">
                    <span class="text-gray-800">Placeholder</span>
                </div>
            {/if}
        </div>
    </div>

    <div class="lg:w-1/3 mt-5 card p-4">
        {#if !isError}
            <div>
                Video Title:<br />
                {#if !video?.title}
                    <div class="placeholder"></div>
                {:else}
                    {video.title}
                {/if}
            </div>

            <div class="mt-1">
                Uploaded on:<br />
                {#if !video_published_at}
                    <div class="placeholder"></div>
                {:else}
                    {video_published_str}
                {/if}
            </div>

            <div class="mt-1">
                Channel:<br />
                {#if !video?.channel_name}
                    <div class="placeholder"></div>
                {:else}
                    {video.channel_name}
                {/if}
            </div>
        {:else}
            <p class="text-red-500">Error: Video not found.</p>
        {/if}
    </div>

    <div class="lg:w-1/3 mt-5 card p-4">
        <div class="">
            <h3>
                Collapsible Card <button on:click={() => (isOpen = !isOpen)}>
                    {#if isOpen}
                        🔼
                    {:else}
                        🔽
                    {/if}
                </button>
            </h3>
        </div>
        {#if isOpen}
            {#if data.length > 0}
                <table class="w-full">
                    <thead class="">
                        <tr>
                            <th class="py-2 text-left">Type</th>
                            <th class="px-4 py-2 text-left">Name</th>
                            <th class="px-4 py-2 text-left">Size</th>
                            <th class="px-4 py-2 text-left">Action</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each data as row}
                            <tr class="hover:bg-gray-600 hover:rounded">
                                {#if row.IsFolder}
                                    <svg class="w-5 h-5 ml-2 mt-2" fill="#ffffff" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/></svg>
                                {:else}
                                    <svg class="w-5 h-5" fill="#ffffff" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M0 64C0 28.7 28.7 0 64 0L224 0l0 128c0 17.7 14.3 32 32 32l128 0 0 288c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zm384 64l-128 0L256 0 384 128z"/></svg>
                                {/if}
                                <td class="px-4 py-2">{row.Name}</td>
                                <td class="px-4 py-2">{row.Size}</td>
                                <td
                                    ><button
                                        class="bg-red-700 p-1 rounded"
                                        data-file={row.ID}
                                        id="trash"
                                    >
                                        <svg
                                            class="w-5 h-5"
                                            fill="#ffffff"
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 448 512"
                                            ><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path
                                                d="M135.2 17.7L128 32 32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0-7.2-14.3C307.4 6.8 296.3 0 284.2 0L163.8 0c-12.1 0-23.2 6.8-28.6 17.7zM416 128L32 128 53.2 467c1.6 25.3 22.6 45 47.9 45l245.8 0c25.3 0 46.3-19.7 47.9-45L416 128z"
                                            /></svg
                                        >
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

    <div class="lg:w-1/3 mt-5 card p-4">
        <button
            class="btn text-center w-full variant-filled"
            {disabled}
            on:click={download_video}
        >
            Download
        </button>
    </div>

    {#if error}
        <p class="text-red-500">{error}</p>
    {/if}

    {#if error_message}
        <p class="text-red-500">{error_message}</p>
    {/if}

    <!-- Use YouTube Embed if API key is not present -->
    {#if !useApi}
        <div>
            <iframe
                title="YT Placeholder"
                width="560"
                height="315"
                src={`https://www.youtube.com/embed/${viewkey}`}
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                allowfullscreen
                style="display: none;"
            ></iframe>
        </div>
    {/if}
</div>
