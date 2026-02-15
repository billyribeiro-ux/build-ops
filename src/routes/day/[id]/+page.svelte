<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { dayPlanCommands, attemptCommands, sessionCommands } from '$lib/commands';
	import TimePlanCard from '$lib/components/time/TimePlanCard.svelte';
	import SessionBlockList from '$lib/components/time/SessionBlockList.svelte';
	import PlanGeneratorModal from '$lib/components/time/PlanGeneratorModal.svelte';
	import Icon from '@iconify/svelte';
	import type { DayPlan, DayAttempt, DaySession, GeneratedPlan } from '$lib/types';

	let dayPlan = $state<DayPlan | null>(null);
	let currentAttempt = $state<DayAttempt | null>(null);
	let sessions = $state<DaySession[]>([]);
	let isLoading = $state(true);
	let showPlanModal = $state(false);
	let error = $state<string | null>(null);

	let dayId = $derived($page.params.id);

	onMount(async () => {
		await loadDayData();
	});

	async function loadDayData() {
		isLoading = true;
		error = null;
		try {
			dayPlan = await dayPlanCommands.get(dayId);
			
			const attempts = await attemptCommands.list(dayId);
			currentAttempt = attempts.find(a => a.status === 'in_progress') || null;
			
			if (!currentAttempt) {
				currentAttempt = await attemptCommands.start(dayId);
			}
			
			if (currentAttempt) {
				sessions = await sessionCommands.list(currentAttempt.id);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load day data';
			console.error('Error loading day data:', err);
		} finally {
			isLoading = false;
		}
	}

	async function handlePlanGenerated(plan: GeneratedPlan) {
		await loadDayData();
	}

	function handlePlanMyDay() {
		showPlanModal = true;
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else if error}
			<div class="rounded-lg border border-red-500/30 bg-red-500/10 p-6 text-center">
				<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
				<p class="mt-3 text-red-400">{error}</p>
			</div>
		{:else if dayPlan && currentAttempt}
			<div class="mb-6 flex items-start justify-between">
				<div>
					<h1 class="text-3xl font-bold text-white">{dayPlan.title}</h1>
					<p class="mt-2 text-gray-400">Day {dayPlan.day_number} • Attempt #{currentAttempt.attempt_number}</p>
				</div>
				<button
					onclick={handlePlanMyDay}
					class="flex items-center gap-2 rounded-lg bg-gradient-to-r from-blue-600 to-purple-600 px-6 py-3 font-semibold text-white shadow-lg transition hover:from-blue-700 hover:to-purple-700"
				>
					<Icon icon="ph:magic-wand-bold" width="20" />
					Plan My Day
				</button>
			</div>

			<div class="grid gap-6 lg:grid-cols-3">
				<div class="lg:col-span-1">
					<TimePlanCard {dayPlan} />
					
					<div class="mt-6 rounded-lg border border-gray-700 bg-gray-800 p-6">
						<h3 class="mb-3 text-lg font-semibold text-white">Implementation Brief</h3>
						<div class="prose prose-invert prose-sm max-w-none">
							<p class="text-gray-300">{dayPlan.implementation_brief}</p>
						</div>
					</div>

					{#if dayPlan.syntax_targets}
						<div class="mt-6 rounded-lg border border-gray-700 bg-gray-800 p-6">
							<h3 class="mb-3 text-lg font-semibold text-white">Syntax Targets</h3>
							<div class="prose prose-invert prose-sm max-w-none">
								<p class="text-gray-300">{dayPlan.syntax_targets}</p>
							</div>
						</div>
					{/if}
				</div>

				<div class="lg:col-span-2">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-2xl font-bold text-white">Today's Sessions</h2>
						{#if sessions.length === 0}
							<button
								onclick={handlePlanMyDay}
								class="flex items-center gap-2 rounded-lg border border-blue-500 bg-blue-500/10 px-4 py-2 text-sm font-medium text-blue-400 hover:bg-blue-500/20"
							>
								<Icon icon="ph:plus-bold" width="16" />
								Generate Schedule
							</button>
						{/if}
					</div>
					
					<SessionBlockList {sessions} />

					<div class="mt-6 rounded-lg border border-gray-700 bg-gray-800 p-6">
						<h3 class="mb-3 text-lg font-semibold text-white">Quick Actions</h3>
						<div class="grid grid-cols-2 gap-3">
							<button class="flex items-center justify-center gap-2 rounded-lg border border-gray-700 bg-gray-900 px-4 py-3 text-white hover:bg-gray-800">
								<Icon icon="ph:code-bold" width="20" />
								Open Editor
							</button>
							<button class="flex items-center justify-center gap-2 rounded-lg border border-gray-700 bg-gray-900 px-4 py-3 text-white hover:bg-gray-800">
								<Icon icon="ph:notebook-bold" width="20" />
								Add Notes
							</button>
							<button class="flex items-center justify-center gap-2 rounded-lg border border-gray-700 bg-gray-900 px-4 py-3 text-white hover:bg-gray-800">
								<Icon icon="ph:image-bold" width="20" />
								Add Evidence
							</button>
							<button class="flex items-center justify-center gap-2 rounded-lg border border-gray-700 bg-gray-900 px-4 py-3 text-white hover:bg-gray-800">
								<Icon icon="ph:check-square-bold" width="20" />
								Submit Day
							</button>
						</div>
					</div>
				</div>
			</div>

			{#if currentAttempt}
				<PlanGeneratorModal
					bind:isOpen={showPlanModal}
					dayPlanId={dayPlan.id}
					dayAttemptId={currentAttempt.id}
					onPlanGenerated={handlePlanGenerated}
				/>
			{/if}
		{/if}
	</div>
</div>
