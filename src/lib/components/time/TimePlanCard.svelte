<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { DayPlan } from '$lib/types';

	let { dayPlan }: { dayPlan: DayPlan } = $props();

	let focusBlocks = $derived.by(() => {
		try {
			return JSON.parse(dayPlan.focus_blocks);
		} catch {
			return [];
		}
	});

	let complexityColor = $derived.by(() => {
		switch (dayPlan.complexity_level) {
			case 1:
			case 2:
				return 'text-green-400';
			case 3:
				return 'text-yellow-400';
			case 4:
			case 5:
				return 'text-red-400';
			default:
				return 'text-gray-400';
		}
	});
</script>

<div class="rounded-lg border border-gray-700 bg-gray-800 p-6">
	<div class="mb-4 flex items-start justify-between">
		<div>
			<h3 class="text-xl font-bold text-white">{dayPlan.title}</h3>
			<p class="mt-1 text-sm text-gray-400">Day {dayPlan.day_number}</p>
		</div>
		<div class="flex items-center gap-2">
			<Icon icon="ph:gauge-bold" width="20" class={complexityColor} />
			<span class="text-sm font-medium {complexityColor}">
				Level {dayPlan.complexity_level}
			</span>
		</div>
	</div>

	<div class="mb-4 grid grid-cols-3 gap-4">
		<div class="rounded-lg bg-gray-900 p-3">
			<div class="text-xs text-gray-400">Minimum</div>
			<div class="mt-1 text-lg font-semibold text-white">{dayPlan.min_minutes} min</div>
		</div>
		<div class="rounded-lg bg-blue-500/10 p-3">
			<div class="text-xs text-blue-400">Recommended</div>
			<div class="mt-1 text-lg font-semibold text-blue-400">{dayPlan.recommended_minutes} min</div>
		</div>
		<div class="rounded-lg bg-purple-500/10 p-3">
			<div class="text-xs text-purple-400">Deep Work</div>
			<div class="mt-1 text-lg font-semibold text-purple-400">{dayPlan.deep_minutes} min</div>
		</div>
	</div>

	{#if focusBlocks.length > 0}
		<div>
			<h4 class="mb-2 text-sm font-medium text-gray-400">Suggested Focus Blocks</h4>
			<div class="space-y-2">
				{#each focusBlocks as block}
					<div class="flex items-center justify-between rounded-lg bg-gray-900 px-3 py-2">
						<span class="text-sm capitalize text-white">{block.session_type}</span>
						<span class="text-sm font-medium text-gray-400">{block.minutes} min</span>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
