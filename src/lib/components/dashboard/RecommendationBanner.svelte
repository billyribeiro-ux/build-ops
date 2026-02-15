<script lang="ts">
	// TODO: Add recommendation and analytics commands when implemented
	// import { getRecommendations, getAnalytics } from '$lib/commands';
	import Icon from '@iconify/svelte';
	import type { TimeRecommendation } from '$lib/types';
	import { onMount } from 'svelte';

	let recommendations = $state<TimeRecommendation[]>([]);
	let isLoading = $state(true);

	onMount(async () => {
		await loadRecommendations();
	});

	async function loadRecommendations() {
		try {
			// TODO: Implement when recommendation commands are available
			// recommendations = await getRecommendations();
			recommendations = [];
		} catch (error) {
			console.error('Failed to load recommendations:', error);
		} finally {
			isLoading = false;
		}
	}

	async function handleApply(id: string) {
		try {
			// TODO: Implement when recommendation commands are available
			// await applyRecommendation(id);
			await loadRecommendations();
		} catch (error) {
			console.error('Failed to apply recommendation:', error);
		}
	}

	async function handleDismiss(id: string) {
		try {
			// TODO: Implement when recommendation commands are available
			// await dismissRecommendation(id);
			await loadRecommendations();
		} catch (error) {
			console.error('Failed to dismiss recommendation:', error);
		}
	}

	function getIconForType(type: string) {
		switch (type) {
			case 'increase_build':
				return 'ph:arrow-up-bold';
			case 'decrease_deep':
				return 'ph:arrow-down-bold';
			case 'add_buffer':
				return 'ph:plus-bold';
			case 'adjust_break':
				return 'ph:coffee-bold';
			default:
				return 'ph:lightbulb-bold';
		}
	}
</script>

{#if !isLoading && recommendations.length > 0}
	<div class="space-y-3">
		{#each recommendations as rec (rec.id)}
			<div class="rounded-lg border border-blue-500/30 bg-blue-500/10 p-4">
				<div class="flex items-start gap-3">
					<div class="rounded-full bg-blue-500/20 p-2">
						<Icon icon={getIconForType(rec.recommendation_type)} width="20" class="text-blue-400" />
					</div>
					<div class="flex-1">
						<div class="flex items-start justify-between">
							<div>
								<h4 class="font-semibold text-white">{rec.title}</h4>
								<p class="mt-1 text-sm text-gray-300">{rec.description}</p>
								<div class="mt-2 flex items-center gap-3 text-xs text-gray-400">
									<span>Confidence: {(rec.confidence_score * 100).toFixed(0)}%</span>
									<span>•</span>
									<span>{rec.data_source}</span>
								</div>
							</div>
						</div>
						<div class="mt-3 flex gap-2">
							<button
								onclick={() => handleApply(rec.id)}
								class="rounded-lg bg-blue-600 px-3 py-1.5 text-sm font-medium text-white hover:bg-blue-700"
							>
								Apply
							</button>
							<button
								onclick={() => handleDismiss(rec.id)}
								class="rounded-lg border border-gray-600 px-3 py-1.5 text-sm font-medium text-gray-300 hover:bg-gray-800"
							>
								Dismiss
							</button>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>
{/if}
