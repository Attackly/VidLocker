<script lang="ts">
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

    <div class="lg:w-1/3 mt-8 card p-4">
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

    <div class="lg:w-1/3 mt-8 card p-4">
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

    <div class="lg:w-1/3 mt-8 card p-4">
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
