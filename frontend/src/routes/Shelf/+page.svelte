<script lang="ts">
	import '../../app.css';
	import Modal from '$lib/modal.svelte';


	const ILLEGAL_CHARS = '\0';
	const UNITS = ['Bytes', 'KB', 'MB', 'GB', 'TB'];

	let currentDir = $state('/');
	let newFolderName = $state('');
	let showModal = $state(false);
	let fileToDelete = $state('');
	let version = $state(0); 

	let files = $state({
		loading: true,
		data: [] as any[],
		error: null as string | null
	});

	let breadcrumbs = $derived(() => {
		if (currentDir === '/') return [];
		const parts = currentDir.split('/').filter(Boolean);
        let path = '';
		return parts.map((part) => {
			path += `/${part}`;
			return { name: part, path };
		});
	});

	$effect(() => {
		const dir = currentDir;
		const v = version;

		async function getFiles() {
			files.loading = true;
			files.error = null;
			try {
				const res = await fetch(`/api/files?dir=${encodeURIComponent(dir)}`);
				if (!res.ok) {
					throw new Error(`Request failed with status: ${res.status}`);
				}
				const raw_data = await res.json();
				if (!Array.isArray(raw_data)) {
					throw new Error('API response was not an array.');
				}
				files.data = raw_data;
			} catch (e: any) {
				console.error('Error fetching files:', e);
				files.error = e.message;
			} finally {
				files.loading = false;
			}
		}

		getFiles();
	});

	function formatBytes(bytes: number) {
		if (bytes === 0) return '0 Bytes';
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		return `${(bytes / 1024 ** i).toFixed(2)} ${UNITS[i]}`;
	}

	function navigateTo(path: string) {
		currentDir = path;
	}

	function navigateInto(dirName: string) {
		const newPath = currentDir === '/' ? `/${dirName}` : `${currentDir}/${dirName}`;
		currentDir = newPath;
	}

	async function deleteItem(path: string) {
		try {
			const res = await fetch(`/api/file${path}`, {
				method: 'DELETE'
			});
			if (!res.ok) throw new Error('API call to delete file failed');
			version++;
		} catch (e) {
			console.error(`Failed to delete item: ${path}`, e);
			// TODO: Show a notification to the user about the failure
		}
	}

	async function createNewDir() {
		if (!newFolderName || newFolderName.includes(ILLEGAL_CHARS)) {
			// TODO: Show a notification for invalid name
			newFolderName = '';
			return;
		}
		const fullPath = currentDir === '/' ? `/${newFolderName}` : `${currentDir}/${newFolderName}`;
		try {
			const res = await fetch('/api/files/directory', {
				method: 'POST',
				body: JSON.stringify({ path: fullPath }),
				headers: { 'Content-Type': 'application/json' }
			});
			if (!res.ok) throw new Error('API call to create directory failed');
			newFolderName = ''; // Clear input on success
			version++; // ✅ Trigger a refresh
		} catch (e) {
			console.error('Failed to create directory:', e);
			// TODO: Show a notification to the user
		}
	}

	async function downloadFile(filePath: string) {
		try {
			const res = await fetch(`/api/files/download?filename=${encodeURIComponent(filePath)}`);
			if (!res.ok) throw new Error(`Download failed: ${res.statusText}`);

			const blob = await res.blob();
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			// Extract filename from the full path for the download attribute
			a.download = filePath.split('/').pop() || 'download';
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			window.URL.revokeObjectURL(url);
		} catch (error) {
			console.error('Error downloading file:', error);
			// TODO: Show a notification to the user
		}
	}

	// --- Modal Logic ---
	function handleDeleteClick(path: string) {
		fileToDelete = path;
		showModal = true;
	}

	function confirmDelete() {
		if (fileToDelete) {
			deleteItem(fileToDelete);
		}
		showModal = false;
		fileToDelete = '';
	}

	function cancelDelete() {
		showModal = false;
		fileToDelete = '';
	}
</script>

<div class="sm:w-3/4 lg:w-1/2 mt-5 p-3 mb-3 text-primary rounded-lg overflow-hidden shadow-lg card-bg relative justify-center flex mx-auto">
	<div class="flex flex-col w-full">
		<ol class="flex w-full flex-wrap items-center rounded-md px-4 py-2 text-primary text-lg">
			<li class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg">
				<div class="mr-2">Current Directory:</div>
				<button type="button" onclick={() => navigateTo('/')}> /</button>
			</li>
			{#each breadcrumbs() as crumb}
				<li class="flex cursor-pointer items-center transition-colors duration-300 hover:text-slate-800 text-primary text-lg">
					<span class="pointer-events-none mx-2 text-primary text-lg"> > </span>
					<button type="button" onclick={() => navigateTo(crumb.path)}>
						{crumb.name}
					</button>
				</li>
			{/each}
		</ol>

		<div class="pb-4"></div>

		{#if files.loading}
			<div class="text-center p-4">Loading files...</div>
		{:else if files.error}
			<div class="p-4 text-sm text-red-800 rounded-lg bg-red-50" role="alert">
				Error loading files: {files.error}
			</div>
		{:else if files.data.length === 0}
			<div class="p-4 text-sm text-yellow-800 rounded-lg bg-yellow-50" role="alert">
				This folder is empty.
			</div>
		{:else}
			<table class="table-fixed w-full">
				<thead class="border-b">
					<tr>
						<th class="w-[5%] px-4 py-2 text-left">Type</th>
						<th class="w-[70%] px-4 py-2 text-left">Name</th>
						<th class="w-[15%] px-4 py-2 text-left">Size</th>
						<th class="w-[10%] px-4 py-2 text-center">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each files.data as row (row.path)}
						<tr class="row-bg-1 odd:row-bg-2">
							<td class="px-2 py-2">
								{#if row.is_directory}
									<svg class="svg-primary w-6 h-6" viewBox="0 0 512 512">
										<path d="M64 480H448c35.3 0 64-28.7 64-64V160c0-35.3-28.7-64-64-64H288c-10.1 0-19.6-4.7-25.6-12.8L243.2 57.6C231.1 41.5 212.1 32 192 32H64C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64z"/>
									</svg>
								{:else}
									<svg class="svg-primary w-6 h-6" viewBox="0 0 384 512">
										<path d="M0 64C0 28.7 28.7 0 64 0H224V128c0 17.7 14.3 32 32 32H384V448c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V64zm384 64H256V0L384 128z"/>
									</svg>
								{/if}
							</td>
							<td class="px-4 py-2 truncate">
								{#if row.is_directory}
									<button class="text-left hover:underline" type="button" onclick={() => navigateInto(row.name)}>
										{row.name}
									</button>
								{:else}
									{row.name}
								{/if}
							</td>
							<td class="px-4 py-2">{formatBytes(row.file_size)}</td>
							<td class="text-center">
								<div class="flex items-center justify-center gap-2">
									<button class="bg-red-700 p-1 rounded cursor-pointer" title="Delete" onclick={() => handleDeleteClick(row.path)} aria-label="Delete">
										<svg class="w-5 h-5" fill="#ffffff" viewBox="0 0 448 512">
											<path d="M135.2 17.7L128 32 32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0-7.2-14.3C307.4 6.8 296.3 0 284.2 0L163.8 0c-12.1 0-23.2 6.8-28.6 17.7zM416 128L32 128 53.2 467c1.6 25.3 22.6 45 47.9 45l245.8 0c25.3 0 46.3-19.7 47.9-45L416 128z"/>
										</svg>
									</button>
									{#if !row.is_directory}
										<button class="bg-blue-800 p-1 rounded cursor-pointer" title="Download" onclick={() => downloadFile(row.path)} aria-label="Download">
											<svg class="w-5 h-5" fill="#ffffff" viewBox="0 0 512 512">
												<path d="M288 32c0-17.7-14.3-32-32-32s-32 14.3-32 32V274.7L150.6 201.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l128 128c12.5 12.5 32.8 12.5 45.3 0l128-128c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L288 274.7V32zM64 352c-35.3 0-64 28.7-64 64v32c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V416c0-35.3-28.7-64-64-64H346.5l-45.3 45.3c-25 25-65.5 25-90.5 0L165.5 352H64zm368 56a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"/>
											</svg>
										</button>
									{/if}
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}

		<!-- Create Folder Form -->
		<div class="pt-4">
			<label for="newFolderName" class="block mb-1">Folder Name</label>
			<div class="flex w-full gap-2">
				<input class="w-2/3 border rounded p-2 input-bg" id="newFolderName" bind:value={newFolderName} onkeydown={(e) => e.key === 'Enter' && createNewDir()}/>
				<button class="w-1/3 button-bg cursor-pointer rounded" type="button" onclick={createNewDir}>
					Create new Folder
				</button>
			</div>
		</div>
	</div>
</div>

<Modal open={showModal} onConfirm={confirmDelete} onCancel={cancelDelete} message="Are you sure you want to delete this item? This action cannot be undone."/>
