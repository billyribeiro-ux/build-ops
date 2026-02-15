<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';

	interface Props {
		isOpen?: boolean;
		onClose?: () => void;
	}

	let { isOpen = $bindable(false), onClose }: Props = $props();

	let searchQuery = $state('');
	let selectedIndex = $state(0);

	const commands = [
		{ id: 'new-program', label: 'Create New Program', icon: 'ph:plus-bold', action: () => goto('/programs/new') },
		{ id: 'programs', label: 'View Programs', icon: 'ph:folder-bold', action: () => goto('/programs') },
		{ id: 'dashboard', label: 'Go to Dashboard', icon: 'ph:house-bold', action: () => goto('/') },
		{ id: 'analytics', label: 'View Analytics', icon: 'ph:chart-bar-bold', action: () => goto('/analytics') },
		{ id: 'search', label: 'Search', icon: 'ph:magnifying-glass-bold', action: () => goto('/search') },
		{ id: 'settings', label: 'Settings', icon: 'ph:gear-bold', action: () => goto('/settings') },
		{ id: 'import', label: 'Import Content', icon: 'ph:upload-bold', action: () => goto('/import') },
		{ id: 'export', label: 'Export Data', icon: 'ph:download-bold', action: () => goto('/export') },
		{ id: 'evidence', label: 'Evidence Locker', icon: 'ph:folder-open-bold', action: () => goto('/evidence') },
	];

	let filteredCommands = $derived(
		searchQuery.trim()
			? commands.filter(cmd =>
				cmd.label.toLowerCase().includes(searchQuery.toLowerCase())
			)
			: commands
	);

	onMount(() => {
		function handleKeyDown(e: KeyboardEvent) {
			if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
				e.preventDefault();
				isOpen = !isOpen;
			}

			if (!isOpen) return;

			if (e.key === 'Escape') {
				e.preventDefault();
				handleClose();
			} else if (e.key === 'ArrowDown') {
				e.preventDefault();
				selectedIndex = (selectedIndex + 1) % filteredCommands.length;
			} else if (e.key === 'ArrowUp') {
				e.preventDefault();
				selectedIndex = selectedIndex === 0 ? filteredCommands.length - 1 : selectedIndex - 1;
			} else if (e.key === 'Enter') {
				e.preventDefault();
				executeCommand(filteredCommands[selectedIndex]);
			}
		}

		document.addEventListener('keydown', handleKeyDown);
		return () => document.removeEventListener('keydown', handleKeyDown);
	});

	function handleClose() {
		isOpen = false;
		searchQuery = '';
		selectedIndex = 0;
		onClose?.();
	}

	function executeCommand(command: typeof commands[0]) {
		if (command) {
			command.action();
			handleClose();
		}
	}

	$effect(() => {
		if (isOpen) {
			selectedIndex = 0;
		}
	});
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-50 flex items-start justify-center bg-black/50 pt-32"
		onclick={handleClose}
	>
		<div
			class="w-full max-w-2xl rounded-xl border border-gray-700 bg-gray-900 shadow-2xl"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="flex items-center gap-3 border-b border-gray-700 px-4 py-3">
				<Icon icon="ph:magnifying-glass-bold" width="20" class="text-gray-400" />
				<input
					type="text"
					bind:value={searchQuery}
					placeholder="Type a command or search..."
					class="flex-1 bg-transparent text-white placeholder-gray-500 focus:outline-none"
					autofocus
				/>
				<kbd class="rounded bg-gray-800 px-2 py-1 text-xs text-gray-400">ESC</kbd>
			</div>

			<div class="max-h-96 overflow-y-auto p-2">
				{#if filteredCommands.length === 0}
					<div class="py-12 text-center">
						<Icon icon="ph:magnifying-glass-bold" width="48" class="mx-auto text-gray-600" />
						<p class="mt-3 text-gray-400">No commands found</p>
					</div>
				{:else}
					{#each filteredCommands as command, index (command.id)}
						<button
							onclick={() => executeCommand(command)}
							onmouseenter={() => selectedIndex = index}
							class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-left transition-colors {index === selectedIndex ? 'bg-blue-500/10 text-blue-400' : 'text-gray-300 hover:bg-gray-800'}"
						>
							<Icon icon={command.icon} width="20" />
							<span class="flex-1">{command.label}</span>
							{#if index === selectedIndex}
								<Icon icon="ph:arrow-bend-down-left-bold" width="16" class="text-gray-500" />
							{/if}
						</button>
					{/each}
				{/if}
			</div>

			<div class="border-t border-gray-700 px-4 py-3">
				<div class="flex items-center justify-between text-xs text-gray-500">
					<div class="flex gap-4">
						<span><kbd class="rounded bg-gray-800 px-1.5 py-0.5">↑↓</kbd> Navigate</span>
						<span><kbd class="rounded bg-gray-800 px-1.5 py-0.5">↵</kbd> Select</span>
						<span><kbd class="rounded bg-gray-800 px-1.5 py-0.5">ESC</kbd> Close</span>
					</div>
					<span>⌘K to toggle</span>
				</div>
			</div>
		</div>
	</div>
{/if}
