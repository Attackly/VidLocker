<script lang="ts">
    let title = "";
    let uploader = "";
    let date: string;

    async function getYouTubeTitle(viewkey: string) {
        const response = await fetch("/api/yt/getTitle", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: '{"url": "' + viewkey + '"}',
        });
        const data = await response.json();

        if (data.video == null) {
            let isError = true;
            let disabled = true;
        } else {
            let isError = false;
        }
        let video = data.video;
        title = data.video.title;
        uploader = data.video.channel_name;
        let video_published_at = new Date(data.video.published_at);

        date = video_published_at.toLocaleString();
    }
</script>

<div
    class="sm:w-3/4 mt-5 p-3 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
    {#if title != ""}
        <h2 class="text-lg font-bold">{title}</h2>
    {:else}
        <div class="w-64 h-5 bg-gray-300 rounded animate-pulse"></div>
    {/if}
    <p class="text-md text-gray-300">
        Uploaded by: <span class="font-semibold">{uploader}</span>
    </p>
    <p class="text-md text-gray-300">
        Upload Date: <span class="font-semibold">{date}</span>
    </p>
</div>
