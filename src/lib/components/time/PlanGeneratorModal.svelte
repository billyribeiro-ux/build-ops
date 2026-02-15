<script lang="ts">
	// TODO: Add time planning commands when implemented
// import { generateTimePlan } from '$lib/commands';
	import Icon from '@iconify/svelte';
	import type { GeneratedPlan } from '$lib/types';

	let {
		isOpen = $bindable(),
		dayPlanId,
		dayAttemptId,
		onPlanGenerated
	}: {
		isOpen: boolean;
		dayPlanId: string;
		dayAttemptId: string;
		onPlanGenerated: (plan: GeneratedPlan) => void;
	} = $props();

	let isGenerating = $state(false);
	let generatedPlan = $state<GeneratedPlan | null>(null);
	let error = $state<string | null>(null);

	async function handleGenerate() {
		isGenerating = true;
		error = null;
		try {
			// TODO: Implement when time planning commands are available
			// generatedPlan = await generateTimePlan(dayPlanId, dayAttemptId);
			// onPlanGenerated(generatedPlan);
			isOpen = false;
		} catch (e) {
			error = `Failed to generate plan: ${e}`;
		} finally {
			isGenerating = false;
		}
	}

	function handleAccept() {
		if (generatedPlan) {
			onPlanGenerated(generatedPlan);
			isOpen = false;
		}
	}

	function handleClose() {
		isOpen = false;
		generatedPlan = null;
		error = null;
	}

	function formatTime(isoString: string) {
		return new Date(isoString).toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4">
		<div class="w-full max-w-2xl rounded-lg border border-gray-700 bg-gray-900 shadow-xl">
			<div class="flex items-center justify-between border-b border-gray-700 p-6">
				<h2 class="text-xl font-bold text-white">Plan My Day</h2>
				<button
					onclick={handleClose}
					class="text-gray-400 hover:text-white"
					aria-label="Close"
				>
					<Icon icon="ph:x-bold" width="24" />
				</button>
			</div>

			<div class="p-6">
				{#if !generatedPlan && !error}
					<div class="text-center">
						<Icon icon="ph:calendar-check-bold" width="64" class="mx-auto text-blue-500" />
						<h3 class="mt-4 text-lg font-semibold text-white">Generate Your Daily Schedule</h3>
						<p class="mt-2 text-sm text-gray-400">
							We'll create a personalized time plan based on your capacity profile, the day's
							complexity, and your historical performance.
						</p>
						<button
							onclick={handleGenerate}
							disabled={isGenerating}
							class="mt-6 flex items-center justify-center gap-2 rounded-lg bg-blue-600 px-6 py-3 font-medium text-white hover:bg-blue-700 disabled:opacity-50"
						>
							{#if isGenerating}
								<Icon icon="ph:spinner-bold" width="20" class="animate-spin" />
								Generating...
							{:else}
								<Icon icon="ph:magic-wand-bold" width="20" />
								Generate Plan
							{/if}
						</button>
					</div>
				{:else if error}
					<div class="rounded-lg bg-red-500/10 p-4 text-center">
						<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
						<p class="mt-2 text-red-400">{error}</p>
						<button
							onclick={handleGenerate}
							class="mt-4 rounded-lg bg-red-600 px-4 py-2 text-white hover:bg-red-700"
						>
							Try Again
						</button>
					</div>
				{:else if generatedPlan}
					<div>
						<div class="mb-4 rounded-lg bg-blue-500/10 p-4">
							<div class="flex items-center justify-between">
								<div>
									<div class="text-sm text-blue-400">Total Time</div>
									<div class="text-2xl font-bold text-white">{generatedPlan.total_minutes} min</div>
								</div>
								<div class="text-right">
									<div class="text-sm text-blue-400">Estimated End</div>
									<div class="text-lg font-semibold text-white">
										{formatTime(generatedPlan.estimated_end_time)}
									</div>
								</div>
							</div>
						</div>

						<div class="space-y-2">
							{#each generatedPlan.sessions as session, i (i)}
								<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
									<div class="flex items-start justify-between">
										<div class="flex-1">
											<h4 class="font-semibold capitalize text-white">{session.session_type}</h4>
											<p class="mt-1 text-sm text-gray-400">{session.description}</p>
										</div>
										<div class="ml-4 text-right">
											<div class="text-sm text-gray-400">
												{formatTime(session.start_time)} - {formatTime(session.end_time)}
											</div>
											<div class="mt-1 text-sm font-medium text-white">
												{session.planned_minutes} min
											</div>
										</div>
									</div>
								</div>
							{/each}
						</div>

						<div class="mt-6 flex gap-3">
							<button
								onclick={handleClose}
								class="flex-1 rounded-lg border border-gray-700 px-4 py-2 text-white hover:bg-gray-800"
							>
								Cancel
							</button>
							<button
								onclick={handleAccept}
								class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-white hover:bg-green-700"
							>
								<Icon icon="ph:check-bold" width="20" />
								Accept Plan
							</button>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}
