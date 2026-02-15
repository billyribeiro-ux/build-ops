<script lang="ts">
	// TODO: Add analytics commands when implemented
// import { getTimeAnalytics } from '$lib/commands';
	import Icon from '@iconify/svelte';
	import type { TimeAnalytics } from '$lib/types';
	import { onMount } from 'svelte';

	let analytics = $state<TimeAnalytics | null>(null);
	let isLoading = $state(true);

	onMount(async () => {
		try {
			// TODO: Implement when analytics commands are available
			// analytics = await getTimeAnalytics();
			analytics = null;
		} catch (error) {
			console.error('Failed to load analytics:', error);
		} finally {
			isLoading = false;
		}
	});

	let todayVariance = $derived.by(() => {
		if (!analytics) return 0;
		if (analytics.today_planned === 0) return 0;
		return ((analytics.today_actual - analytics.today_planned) / analytics.today_planned) * 100;
	});

	let weekProgress = $derived.by(() => {
		if (!analytics) return 0;
		if (analytics.week_target === 0) return 0;
		return (analytics.week_total / analytics.week_target) * 100;
	});
</script>

<div class="space-y-4">
	<div class="grid grid-cols-2 gap-4">
		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
			<div class="flex items-center gap-2 text-sm text-gray-400">
				<Icon icon="ph:clock-bold" width="16" />
				Today's Plan
			</div>
			{#if isLoading}
				<div class="mt-2 h-8 w-20 animate-pulse rounded bg-gray-700"></div>
			{:else if analytics}
				<div class="mt-2 text-2xl font-bold text-white">{analytics.today_actual} min</div>
				<div class="mt-1 text-xs text-gray-400">
					of {analytics.today_planned} min
					{#if todayVariance !== 0}
						<span class:text-green-400={todayVariance > 0} class:text-red-400={todayVariance < 0}>
							({todayVariance > 0 ? '+' : ''}{todayVariance.toFixed(0)}%)
						</span>
					{/if}
				</div>
			{/if}
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
			<div class="flex items-center gap-2 text-sm text-gray-400">
				<Icon icon="ph:calendar-bold" width="16" />
				This Week
			</div>
			{#if isLoading}
				<div class="mt-2 h-8 w-20 animate-pulse rounded bg-gray-700"></div>
			{:else if analytics}
				<div class="mt-2 text-2xl font-bold text-white">{analytics.week_total} min</div>
				<div class="mt-1 h-1.5 overflow-hidden rounded-full bg-gray-700">
					<div
						class="h-full bg-gradient-to-r from-blue-500 to-purple-500"
						style="width: {Math.min(weekProgress, 100)}%"
					></div>
				</div>
				<div class="mt-1 text-xs text-gray-400">
					{weekProgress.toFixed(0)}% of {analytics.week_target} min target
				</div>
			{/if}
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
			<div class="flex items-center gap-2 text-sm text-gray-400">
				<Icon icon="ph:target-bold" width="16" />
				Focus Efficiency
			</div>
			{#if isLoading}
				<div class="mt-2 h-8 w-20 animate-pulse rounded bg-gray-700"></div>
			{:else if analytics}
				<div class="mt-2 text-2xl font-bold text-white">
					{analytics.focus_efficiency.toFixed(0)}%
				</div>
				<div class="mt-1 text-xs text-gray-400">Deep work ratio</div>
			{/if}
		</div>

		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
			<div class="flex items-center gap-2 text-sm text-gray-400">
				<Icon icon="ph:fire-bold" width="16" />
				Burnout Guard
			</div>
			{#if isLoading}
				<div class="mt-2 h-8 w-20 animate-pulse rounded bg-gray-700"></div>
			{:else if analytics}
				<div class="mt-2 text-2xl font-bold text-white">
					{analytics.deep_days_used}/{analytics.deep_days_limit}
				</div>
				<div class="mt-1 text-xs text-gray-400">Deep days this week</div>
			{/if}
		</div>
	</div>

	{#if analytics && analytics.accuracy_trend.length > 0}
		<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
			<h3 class="mb-3 text-sm font-medium text-gray-400">Time Accuracy Trend (14 days)</h3>
			<div class="space-y-2">
				{#each analytics.accuracy_trend.slice(-7) as point}
					<div class="flex items-center gap-3">
						<div class="w-16 text-xs text-gray-500">
							{new Date(point.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}
						</div>
						<div class="flex-1">
							<div class="flex items-center gap-2">
								<div class="h-2 flex-1 overflow-hidden rounded-full bg-gray-700">
									<div
										class="h-full bg-blue-500"
										style="width: {Math.min((point.actual / point.planned) * 100, 100)}%"
									></div>
								</div>
								<div class="w-20 text-right text-xs text-gray-400">
									{point.actual}/{point.planned} min
								</div>
							</div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{/if}
</div>
