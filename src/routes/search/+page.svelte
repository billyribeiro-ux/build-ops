<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { listPrograms, listDayPlans, listAttempts } from '$lib/commands';
	import type { ProgramSummary, DayPlanSummary, DayAttemptSummary } from '$lib/types';

	interface SearchResult {
		id: string;
		title: string;
		description: string;
		category: 'program' | 'day' | 'attempt';
		date: string;
		url: string;
	}

	let searchQuery = $state('');
	let searchResults = $state<SearchResult[]>([]);
	let isSearching = $state(false);
	let selectedCategory = $state<'all' | 'programs' | 'days' | 'attempts'>('all');

	let allPrograms = $state<ProgramSummary[]>([]);
	let allDays = $state<DayPlanSummary[]>([]);
	let allAttempts = $state<DayAttemptSummary[]>([]);
	let dataLoaded = $state(false);

	onMount(() => {
		document.addEventListener('keydown', handleKeyDown);
		loadSearchData();
		return () => document.removeEventListener('keydown', handleKeyDown);
	});

	async function loadSearchData() {
		try {
			allPrograms = await listPrograms();
			const dayResults = await Promise.all(
				allPrograms.map((p) => listDayPlans(p.id).catch(() => []))
			);
			allDays = dayResults.flat();
			const attemptResults = await Promise.all(
				allPrograms.map((p) => listAttempts(p.id).catch(() => []))
			);
			allAttempts = attemptResults.flat();
			dataLoaded = true;
		} catch (err) {
			console.error('Failed to load search data:', err);
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
			e.preventDefault();
			const input = document.getElementById('search-input') as HTMLInputElement;
			input?.focus();
		}
	}

	function handleSearch() {
		if (!searchQuery.trim()) {
			searchResults = [];
			return;
		}

		isSearching = true;
		const q = searchQuery.toLowerCase();
		const results: SearchResult[] = [];

		if (selectedCategory === 'all' || selectedCategory === 'programs') {
			for (const p of allPrograms) {
				if (p.title.toLowerCase().includes(q)) {
					results.push({
						id: p.id,
						title: p.title,
						description: `${p.days_count} days, ${p.completed_days} completed`,
						category: 'program',
						date: '',
						url: `/programs/${p.id}`
					});
				}
			}
		}

		if (selectedCategory === 'all' || selectedCategory === 'days') {
			for (const d of allDays) {
				if (d.title.toLowerCase().includes(q)) {
					results.push({
						id: d.id,
						title: `Day ${d.day_number}: ${d.title}`,
						description: `Status: ${d.status}`,
						category: 'day',
						date: '',
						url: `/day/${d.id}`
					});
				}
			}
		}

		if (selectedCategory === 'all' || selectedCategory === 'attempts') {
			for (const a of allAttempts) {
				if (a.day_title.toLowerCase().includes(q)) {
					results.push({
						id: a.id,
						title: `Attempt #${a.attempt_number}: ${a.day_title}`,
						description: `Score: ${a.total_score}/100 - ${a.status}`,
						category: 'attempt',
						date: new Date(a.created_at).toLocaleDateString(),
						url: `/work/${a.id}`
					});
				}
			}
		}

		searchResults = results;
		isSearching = false;
	}

	function getCategoryIcon(category: string) {
		switch (category) {
			case 'program': return 'ph:folder-bold';
			case 'day': return 'ph:calendar-bold';
			case 'attempt': return 'ph:code-bold';
			default: return 'ph:file-bold';
		}
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-4xl">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-white">Search</h1>
			<p class="mt-1 text-gray-400">Find anything across your programs, days, and attempts</p>
		</div>

		<Card>
			<div class="p-6">
				<div class="mb-4 flex items-center gap-2 rounded-lg border border-gray-700 bg-gray-800 px-4 py-3">
					<Icon icon="ph:magnifying-glass-bold" width="20" class="text-gray-400" />
					<input
						id="search-input"
						type="text"
						bind:value={searchQuery}
						oninput={handleSearch}
						placeholder="Search... (⌘K)"
						class="flex-1 bg-transparent text-white placeholder-gray-500 focus:outline-none"
					/>
					{#if searchQuery}
						<button
							onclick={() => { searchQuery = ''; searchResults = []; }}
							class="text-gray-400 hover:text-white"
						>
							<Icon icon="ph:x-bold" width="16" />
						</button>
					{/if}
				</div>

				<div class="mb-4 flex gap-2">
					<button
						onclick={() => selectedCategory = 'all'}
						class="rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {selectedCategory === 'all' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
					>
						All
					</button>
					<button
						onclick={() => selectedCategory = 'programs'}
						class="rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {selectedCategory === 'programs' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
					>
						Programs
					</button>
					<button
						onclick={() => selectedCategory = 'days'}
						class="rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {selectedCategory === 'days' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
					>
						Day Plans
					</button>
					<button
						onclick={() => selectedCategory = 'attempts'}
						class="rounded-lg px-3 py-1.5 text-sm font-medium transition-colors {selectedCategory === 'attempts' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
					>
						Attempts
					</button>
				</div>

				{#if isSearching}
					<div class="flex h-48 items-center justify-center">
						<Icon icon="ph:spinner-bold" width="32" class="animate-spin text-blue-500" />
					</div>
				{:else if searchQuery && searchResults.length === 0}
					<div class="py-12 text-center">
						<Icon icon="ph:magnifying-glass-bold" width="48" class="mx-auto text-gray-600" />
						<p class="mt-3 text-gray-400">No results found for "{searchQuery}"</p>
						<p class="mt-1 text-sm text-gray-500">Try adjusting your search terms</p>
					</div>
				{:else if searchResults.length > 0}
					<div class="space-y-2">
						{#each searchResults as result (result.id)}
							<button
								onclick={() => goto(result.url)}
								class="w-full rounded-lg border border-gray-700 bg-gray-800 p-4 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
							>
								<div class="flex items-start gap-3">
									<Icon icon={getCategoryIcon(result.category)} width="20" class="mt-0.5 text-blue-500" />
									<div class="flex-1">
										<h3 class="font-medium text-white">{result.title}</h3>
										<p class="mt-1 text-sm text-gray-400">{result.description}</p>
										<div class="mt-2 flex items-center gap-2">
											<Badge variant="info">{result.category}</Badge>
											{#if result.date}
												<span class="text-xs text-gray-500">{result.date}</span>
											{/if}
										</div>
									</div>
								</div>
							</button>
						{/each}
					</div>
				{:else}
					<div class="py-12 text-center">
						<Icon icon="ph:keyboard-bold" width="48" class="mx-auto text-gray-600" />
						<p class="mt-3 text-gray-400">Start typing to search</p>
						<p class="mt-1 text-sm text-gray-500">Press ⌘K to focus search</p>
					</div>
				{/if}
			</div>
		</Card>

		<div class="mt-6">
			<Card>
				<div class="p-6">
					<h2 class="mb-4 text-lg font-semibold text-white">Quick Commands</h2>
					<div class="space-y-2">
						<button onclick={() => goto('/programs/new')} class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750">
							<Icon icon="ph:plus-bold" width="20" class="text-blue-500" />
							<div class="flex-1">
								<p class="text-sm font-medium text-white">Create new program</p>
								<p class="text-xs text-gray-400">Start a new learning curriculum</p>
							</div>
						</button>
						<button onclick={() => goto('/import')} class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750">
							<Icon icon="ph:upload-bold" width="20" class="text-green-500" />
							<div class="flex-1">
								<p class="text-sm font-medium text-white">Import content</p>
								<p class="text-xs text-gray-400">Upload PDFs or documents</p>
							</div>
						</button>
						<button onclick={() => goto('/analytics')} class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750">
							<Icon icon="ph:chart-bar-bold" width="20" class="text-purple-500" />
							<div class="flex-1">
								<p class="text-sm font-medium text-white">View analytics</p>
								<p class="text-xs text-gray-400">Check your progress</p>
							</div>
						</button>
					</div>
				</div>
			</Card>
		</div>
	</div>
</div>
