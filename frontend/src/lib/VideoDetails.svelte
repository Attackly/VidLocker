<script lang="ts">
    import { link } from "$lib/stores/linkstore";

    let video = $state({
		loading: true,
		error: null as string | null,
		title: '',
		uploader: '',
		date: ''
	});

    let viewkey = $derived(() => {
		const newLink = $link;
		if (!newLink) return '';
		if (newLink.includes('?v=')) return newLink.split('?v=')[1];
		if (newLink.length === 11) return newLink;
		return '';
	});
    
    $effect(() => {
		async function getYouTubeTitle() {
			if (!viewkey) {
				video.title = '';
				video.loading = false;
				return;
			}

			video.loading = true;
			video.error = null;
			try {
				const response = await fetch('/api/yt/getTitle', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json' },
					body: JSON.stringify({ url: viewkey })
				});

				if (!response.ok) throw new Error('API request failed');
				const data = await response.json();
				if (!data.video) throw new Error('Video data not found in response.');

				const publishedAt = new Date(data.video.published_at);

				// Update state in a single, clean assignment
				video.title = data.video.title;
				video.uploader = data.video.channel_name;
				video.date = publishedAt.toLocaleString();
			} catch (e: any) {
				console.error('Failed to fetch video details:', e);
				video.error = e.message;
				video.title = 'Error loading video';
			} finally {
				video.loading = false;
			}
		}

		getYouTubeTitle();
	});
</script>

<div
    class="sm:w-3/4 lg:w-1/2 mt-5 p-3 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative"
>
    {#if video.title != ""}
        <h2 class="text-lg font-bold">{video.title}</h2>
    {:else}
        <div class="w-3/5 h-5 bg-gray-300 rounded animate-pulse"></div>
    {/if}
    <p class="text-md text-gray-300">
        Uploaded by: <span class="font-semibold">{video.uploader}</span>
    </p>
    <p class="text-md text-gray-300">
        Upload Date: <span class="font-semibold">{video.date}</span>
    </p>
</div>
