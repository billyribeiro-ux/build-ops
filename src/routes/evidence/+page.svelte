<script lang="ts">
	import { onMount } from 'svelte';
	import { listArtifacts, createArtifact } from '$lib/commands';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Icon from '@iconify/svelte';
	import type { Artifact, CreateArtifactInput } from '$lib/types';

	let artifacts = $state<Artifact[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let isDragging = $state(false);

	onMount(async () => {
		await loadArtifacts();
	});

	async function loadArtifacts() {
		isLoading = true;
		error = null;
		try {
			artifacts = await listArtifacts();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load artifacts';
			console.error('Error loading artifacts:', err);
		} finally {
			isLoading = false;
		}
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
	}

	async function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;

		const files = e.dataTransfer?.files;
		if (!files || files.length === 0) return;

		for (const file of Array.from(files)) {
			try {
				const content = await file.text();
				const input: CreateArtifactInput = {
					day_attempt_id: '', // TODO: Get from context
					artifact_type: 'file',
					title: file.name,
					content: content,
					file_path: file.name
				};
				await createArtifact(input);
			} catch (err) {
				console.error('Error uploading file:', err);
			}
		}

		await loadArtifacts();
	}

	function getArtifactIcon(type: string) {
		switch (type) {
			case 'file': return 'ph:file-bold';
			case 'link': return 'ph:link-bold';
			case 'code': return 'ph:code-bold';
			case 'note': return 'ph:note-bold';
			case 'screenshot': return 'ph:image-bold';
			default: return 'ph:file-bold';
		}
	}

	function getArtifactColor(type: string) {
		switch (type) {
			case 'file': return 'info';
			case 'link': return 'purple';
			case 'code': return 'success';
			case 'note': return 'warning';
			case 'screenshot': return 'danger';
			default: return 'info';
		}
	}

	function formatDate(dateStr: string) {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric'
		});
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-white">Evidence Locker</h1>
			<p class="mt-1 text-gray-400">Store and organize your learning artifacts</p>
		</div>

		<div
			class="mb-6 rounded-xl border-2 border-dashed p-12 text-center transition-colors {isDragging ? 'border-blue-500 bg-blue-500/10' : 'border-gray-700 bg-gray-800/50'}"
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
		>
			<Icon icon="ph:upload-simple-bold" width="64" class="mx-auto text-gray-600" />
			<h3 class="mt-4 text-lg font-semibold text-white">Drop files here</h3>
			<p class="mt-1 text-sm text-gray-400">
				Or click to browse and upload screenshots, code files, or documents
			</p>
			<div class="mt-6 flex justify-center gap-3">
				<Button icon="ph:file-bold">Upload File</Button>
				<Button variant="outline" icon="ph:link-bold">Add Link</Button>
				<Button variant="outline" icon="ph:code-bold">Add Code</Button>
				<Button variant="outline" icon="ph:note-bold">Add Note</Button>
			</div>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else if error}
			<Card>
				<div class="p-12 text-center">
					<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
					<p class="mt-3 text-red-400">{error}</p>
				</div>
			</Card>
		{:else if artifacts.length === 0}
			<Card>
				<div class="p-12 text-center">
					<Icon icon="ph:folder-open-bold" width="48" class="mx-auto text-gray-600" />
					<h2 class="mt-3 text-xl font-semibold text-white">No artifacts yet</h2>
					<p class="mt-1 text-gray-400">Upload files or add links to start building your evidence collection</p>
				</div>
			</Card>
		{:else}
			<div class="grid grid-cols-3 gap-4">
				{#each artifacts as artifact (artifact.id)}
					<Card>
						<div class="p-4">
							<div class="mb-3 flex items-start justify-between">
								<div class="flex items-center gap-2">
									<Icon icon={getArtifactIcon(artifact.artifact_type)} width="20" class="text-blue-500" />
									<Badge variant={getArtifactColor(artifact.artifact_type)}>
										{artifact.artifact_type}
									</Badge>
								</div>
								<button class="text-gray-400 hover:text-white">
									<Icon icon="ph:dots-three-bold" width="20" />
								</button>
							</div>
							
							<h3 class="mb-2 font-semibold text-white line-clamp-2">
								{artifact.title}
							</h3>
							
							{#if artifact.content}
								<p class="mb-3 text-sm text-gray-400 line-clamp-3">
									{artifact.content}
								</p>
							{/if}

							<div class="flex items-center justify-between text-xs text-gray-500">
								<span>{formatDate(artifact.created_at)}</span>
								{#if artifact.file_path}
									<span class="truncate">{artifact.file_path}</span>
								{/if}
							</div>
						</div>
					</Card>
				{/each}
			</div>

			<div class="mt-6 rounded-lg border border-gray-700 bg-gray-800 p-6">
				<h3 class="mb-4 text-lg font-semibold text-white">Summary</h3>
				<div class="grid grid-cols-5 gap-6">
					<div>
						<p class="text-sm text-gray-400">Total</p>
						<p class="mt-1 text-2xl font-bold text-white">{artifacts.length}</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Files</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{artifacts.filter(a => a.artifact_type === 'file').length}
						</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Links</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{artifacts.filter(a => a.artifact_type === 'link').length}
						</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Code</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{artifacts.filter(a => a.artifact_type === 'code').length}
						</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Notes</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{artifacts.filter(a => a.artifact_type === 'note').length}
						</p>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
