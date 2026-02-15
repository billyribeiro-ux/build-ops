<script lang="ts">
	import { onMount } from 'svelte';
	import { programCommands, dayPlanCommands } from '$lib/commands';
	import TimeAnalyticsWidget from '$lib/components/dashboard/TimeAnalyticsWidget.svelte';
	import RecommendationBanner from '$lib/components/dashboard/RecommendationBanner.svelte';
	import Icon from '@iconify/svelte';
	import type { Program, DayPlan } from '$lib/types';

	let programs = $state<Program[]>([]);
	let dayPlansMap = $state<Record<string, DayPlan[]>>({});
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let showCreateForm = $state(false);
	let newTitle = $state('');
	let newDescription = $state('');
	let newTargetDays = $state(30);
	let isCreating = $state(false);

	onMount(async () => {
		await loadPrograms();
	});

	async function loadPrograms() {
		isLoading = true;
		error = null;
		try {
			programs = await programCommands.list();
			for (const program of programs) {
				try {
					dayPlansMap[program.id] = await dayPlanCommands.list(program.id);
				} catch {
					dayPlansMap[program.id] = [];
				}
			}
		} catch (err) {
			error = err instanceof Error ? err.message : String(err);
			console.error('Failed to load programs:', err);
		} finally {
			isLoading = false;
		}
	}

	async function handleCreate() {
		if (!newTitle.trim()) return;
		isCreating = true;
		try {
			await programCommands.create({
				title: newTitle.trim(),
				description: newDescription.trim(),
				target_days: newTargetDays
			});
			newTitle = '';
			newDescription = '';
			newTargetDays = 30;
			showCreateForm = false;
			await loadPrograms();
		} catch (err) {
			alert(`Failed to create program: ${err}`);
		} finally {
			isCreating = false;
		}
	}

	function getDayPlansForProgram(programId: string): DayPlan[] {
		return dayPlansMap[programId] ?? [];
	}

	function getNextDay(programId: string): DayPlan | null {
		const plans = getDayPlansForProgram(programId);
		return plans.find(p => p.status === 'draft' || p.status === 'published') ?? plans[0] ?? null;
	}
</script>

<div class="p-6">
	<div class="mx-auto max-w-6xl">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Dashboard</h1>
				<p class="mt-1 text-gray-400">Your learning programs and time analytics</p>
			</div>
			<div class="flex gap-3">
				<a
					href="/import"
					class="flex items-center gap-2 rounded-lg border border-gray-700 bg-gray-800 px-4 py-2.5 text-sm font-medium text-gray-300 transition hover:bg-gray-700"
				>
					<Icon icon="ph:upload-bold" width="16" />
					Import Curriculum
				</a>
				<button
					onclick={() => showCreateForm = !showCreateForm}
					class="flex items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2.5 text-sm font-semibold text-white transition hover:bg-indigo-700"
				>
					<Icon icon="ph:plus-bold" width="16" />
					New Program
				</button>
			</div>
		</div>

		<RecommendationBanner />

		{#if showCreateForm}
			<div class="mb-6 rounded-xl border border-indigo-500/30 bg-gray-900 p-6">
				<h3 class="mb-4 text-lg font-semibold text-white">Create New Program</h3>
				<div class="space-y-4">
					<div>
						<label for="title" class="mb-1 block text-sm font-medium text-gray-400">Title</label>
						<input
							id="title"
							type="text"
							bind:value={newTitle}
							placeholder="e.g. SvelteKit Mastery"
							class="w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2.5 text-white placeholder-gray-500 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
						/>
					</div>
					<div>
						<label for="desc" class="mb-1 block text-sm font-medium text-gray-400">Description</label>
						<textarea
							id="desc"
							bind:value={newDescription}
							placeholder="What will you learn?"
							rows="2"
							class="w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2.5 text-white placeholder-gray-500 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
						></textarea>
					</div>
					<div>
						<label for="days" class="mb-1 block text-sm font-medium text-gray-400">Target Days</label>
						<input
							id="days"
							type="number"
							bind:value={newTargetDays}
							min="1"
							max="365"
							class="w-32 rounded-lg border border-gray-700 bg-gray-800 px-4 py-2.5 text-white focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
						/>
					</div>
					<div class="flex gap-3">
						<button
							onclick={handleCreate}
							disabled={!newTitle.trim() || isCreating}
							class="rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-indigo-700 disabled:opacity-50"
						>
							{isCreating ? 'Creating...' : 'Create Program'}
						</button>
						<button
							onclick={() => showCreateForm = false}
							class="rounded-lg border border-gray-700 px-5 py-2.5 text-sm font-medium text-gray-400 transition hover:bg-gray-800"
						>
							Cancel
						</button>
					</div>
				</div>
			</div>
		{/if}

		<div class="grid gap-6 lg:grid-cols-3">
			<div class="lg:col-span-2">
				<h2 class="mb-4 text-xl font-bold text-white">Programs</h2>

				{#if isLoading}
					<div class="flex h-48 items-center justify-center">
						<Icon icon="ph:spinner-bold" width="32" class="animate-spin text-indigo-400" />
					</div>
				{:else if error}
					<div class="rounded-lg border border-red-500/30 bg-red-500/10 p-6 text-center">
						<Icon icon="ph:warning-bold" width="32" class="mx-auto text-red-400" />
						<p class="mt-2 text-sm text-red-300">{error}</p>
						<button
							onclick={loadPrograms}
							class="mt-3 rounded-lg bg-red-600/20 px-4 py-2 text-sm font-medium text-red-300 hover:bg-red-600/30"
						>
							Retry
						</button>
					</div>
				{:else if programs.length === 0}
					<div class="rounded-xl border border-dashed border-gray-700 bg-gray-900/50 p-12 text-center">
						<Icon icon="ph:books-bold" width="48" class="mx-auto text-gray-600" />
						<h3 class="mt-4 text-lg font-semibold text-gray-300">No programs yet</h3>
						<p class="mt-2 text-sm text-gray-500">
							Create a new program or import a curriculum to get started.
						</p>
						<div class="mt-6 flex justify-center gap-3">
							<button
								onclick={() => showCreateForm = true}
								class="flex items-center gap-2 rounded-lg bg-indigo-600 px-5 py-2.5 text-sm font-semibold text-white hover:bg-indigo-700"
							>
								<Icon icon="ph:plus-bold" width="16" />
								Create Program
							</button>
							<a
								href="/import"
								class="flex items-center gap-2 rounded-lg border border-gray-700 bg-gray-800 px-5 py-2.5 text-sm font-medium text-gray-300 hover:bg-gray-700"
							>
								<Icon icon="ph:upload-bold" width="16" />
								Import PDF
							</a>
						</div>
					</div>
				{:else}
					<div class="space-y-4">
						{#each programs as program (program.id)}
							{@const nextDay = getNextDay(program.id)}
							{@const dayPlans = getDayPlansForProgram(program.id)}
							<div class="group rounded-xl border border-gray-800 bg-gray-900 p-5 transition hover:border-gray-700">
								<div class="flex items-start justify-between">
									<div class="flex-1">
										<div class="flex items-center gap-3">
											<h3 class="text-lg font-semibold text-white">{program.title}</h3>
											{#if program.status === 'active'}
												<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium capitalize bg-green-500/20 text-green-300">
													<span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
													{program.status}
												</span>
											{:else if program.status === 'paused'}
												<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium capitalize bg-yellow-500/20 text-yellow-300">
													<span class="h-1.5 w-1.5 rounded-full bg-yellow-500"></span>
													{program.status}
												</span>
											{:else if program.status === 'completed'}
												<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium capitalize bg-blue-500/20 text-blue-300">
													<span class="h-1.5 w-1.5 rounded-full bg-blue-500"></span>
													{program.status}
												</span>
											{:else if program.status === 'archived'}
												<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium capitalize bg-gray-500/20 text-gray-300">
													<span class="h-1.5 w-1.5 rounded-full bg-gray-500"></span>
													{program.status}
												</span>
											{:else}
												<span class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium capitalize bg-gray-500/20 text-gray-300">
													<span class="h-1.5 w-1.5 rounded-full bg-gray-500"></span>
													{program.status}
												</span>
											{/if}
										</div>
										{#if program.description}
											<p class="mt-1.5 text-sm text-gray-400">{program.description}</p>
										{/if}
										<div class="mt-3 flex items-center gap-4 text-xs text-gray-500">
											<span class="flex items-center gap-1">
												<Icon icon="ph:calendar-bold" width="14" />
												{program.target_days} day target
											</span>
											<span class="flex items-center gap-1">
												<Icon icon="ph:stack-bold" width="14" />
												{dayPlans.length} day plans
											</span>
											<span class="flex items-center gap-1">
												<Icon icon="ph:clock-bold" width="14" />
												{new Date(program.updated_at).toLocaleDateString()}
											</span>
										</div>
									</div>
									{#if nextDay}
										<a
											href="/day/{nextDay.id}"
											class="flex items-center gap-2 rounded-lg bg-indigo-600/10 px-4 py-2 text-sm font-medium text-indigo-300 transition hover:bg-indigo-600/20"
										>
											<Icon icon="ph:play-bold" width="14" />
											Day {nextDay.day_number}
										</a>
									{/if}
								</div>

								{#if dayPlans.length > 0}
									<div class="mt-4 flex gap-1">
										{#each dayPlans.slice(0, 30) as dp}
											<div
												class="h-2 flex-1 rounded-full {dp.status === 'archived' ? 'bg-green-500' : dp.status === 'published' ? 'bg-indigo-500' : 'bg-gray-700'}"
												title="Day {dp.day_number}: {dp.title}"
											></div>
										{/each}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<div class="space-y-6">
				<div>
					<h2 class="mb-4 text-xl font-bold text-white">Time Analytics</h2>
					<TimeAnalyticsWidget />
				</div>
			</div>
		</div>
	</div>
</div>
