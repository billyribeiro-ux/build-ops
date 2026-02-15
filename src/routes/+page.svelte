<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { listPrograms, getStreak, getBadges, getDueReviews } from '$lib/commands';
	import type { ProgramSummary, Streak, DueReview } from '$lib/types';
	import type { Badge as BadgeType } from '$lib/types/badge';

	let programs = $state<ProgramSummary[]>([]);
	let streaks = $state<Streak[]>([]);
	let badges = $state<BadgeType[]>([]);
	let dueReviews = $state<DueReview[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let totalDaysCompleted = $derived(programs.reduce((sum, p) => sum + p.completed_days, 0));
	let averageScore = $derived(
		programs.length > 0
			? Math.round(programs.reduce((sum, p) => sum + (p.latest_score || 0), 0) / programs.length)
			: 0
	);
	let bestStreak = $derived(
		streaks.length > 0 ? Math.max(...streaks.map((s) => s.current_streak)) : 0
	);
	let longestStreak = $derived(
		streaks.length > 0 ? Math.max(...streaks.map((s) => s.longest_streak)) : 0
	);
	let totalFreezes = $derived(
		streaks.length > 0 ? streaks.reduce((sum, s) => sum + s.freezes_available, 0) : 0
	);

	onMount(async () => {
		await loadDashboard();
	});

	async function loadDashboard() {
		isLoading = true;
		error = null;
		try {
			programs = await listPrograms();
			const streakResults = await Promise.all(
				programs.map((p) => getStreak(p.id).catch(() => null))
			);
			streaks = streakResults.filter((s): s is Streak => s !== null);
			const badgeResults = await Promise.all(
				programs.map((p) => getBadges(p.id).catch(() => []))
			);
			badges = badgeResults.flat();
			const reviewResults = await Promise.all(
				programs.map((p) => getDueReviews(p.id).catch(() => []))
			);
			dueReviews = reviewResults.flat();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load programs';
			console.error('Failed to load dashboard:', err);
		} finally {
			isLoading = false;
		}
	}

	function getScoreColor(score: number) {
		if (score >= 95) return 'text-purple-400';
		if (score >= 70) return 'text-green-400';
		return 'text-yellow-400';
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Dashboard</h1>
				<p class="mt-1 text-gray-400">Your learning progress at a glance</p>
			</div>
			<div class="flex gap-3">
				<Button onclick={() => goto('/programs')} icon="ph:folder-bold">
					Programs
				</Button>
				<Button onclick={() => goto('/import')} variant="outline" icon="ph:upload-bold">
					Import
				</Button>
			</div>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else}
			<div class="mb-8 grid grid-cols-4 gap-4">
				<Card>
					<div class="p-6">
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-medium text-gray-400">Current Streak</h3>
							<Icon icon="ph:fire-bold" width="20" class="text-orange-500" />
						</div>
						<p class="text-4xl font-bold text-white">{bestStreak}</p>
						<p class="mt-1 text-sm text-gray-400">days in a row</p>
						<div class="mt-2 flex items-center gap-3 text-xs text-gray-500">
							<span>Best: {longestStreak}</span>
							{#if totalFreezes > 0}
								<span class="flex items-center gap-1 text-blue-400">
									<Icon icon="ph:snowflake-bold" width="12" />{totalFreezes} freezes
								</span>
							{/if}
						</div>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-medium text-gray-400">Days Completed</h3>
							<Icon icon="ph:check-circle-bold" width="20" class="text-blue-500" />
						</div>
						<p class="text-4xl font-bold text-blue-400">{totalDaysCompleted}</p>
						<p class="mt-1 text-sm text-gray-400">across {programs.length} programs</p>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-medium text-gray-400">Average Score</h3>
							<Icon icon="ph:chart-line-up-bold" width="20" class="text-green-500" />
						</div>
						<p class="text-4xl font-bold {getScoreColor(averageScore)}">{averageScore}</p>
						<p class="mt-1 text-sm text-gray-400">out of 100</p>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-2 flex items-center justify-between">
							<h3 class="text-sm font-medium text-gray-400">Badges Earned</h3>
							<Icon icon="ph:medal-bold" width="20" class="text-yellow-500" />
						</div>
						<p class="text-4xl font-bold text-yellow-400">{badges.length}</p>
						<p class="mt-1 text-sm text-gray-400">achievements unlocked</p>
					</div>
				</Card>
			</div>

			<div class="mb-8 grid grid-cols-3 gap-6">
				<div class="col-span-2">
					<Card>
						<div class="p-6">
							<h2 class="mb-4 text-lg font-semibold text-white">Active Programs</h2>
							{#if programs.length === 0}
								<div class="py-12 text-center">
									<Icon icon="ph:folder-open-bold" width="48" class="mx-auto text-gray-600" />
									<p class="mt-3 text-gray-400">No programs yet</p>
									<Button onclick={() => goto('/programs')} class="mt-4" size="sm">
										Create Program
									</Button>
								</div>
							{:else}
								<div class="space-y-3">
									{#each programs as program (program.id)}
										<button
											onclick={() => goto(`/programs/${program.id}`)}
											class="w-full rounded-lg border border-gray-700 bg-gray-800 p-4 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
										>
											<div class="flex items-start justify-between">
												<div class="flex-1">
													<h3 class="font-semibold text-white">{program.title}</h3>
													<div class="mt-2 flex items-center gap-3">
														<Badge variant="info">{program.days_count} days</Badge>
														<Badge variant="success">{program.completed_days} completed</Badge>
														<span class="text-sm text-gray-400">
															{program.days_count > 0 ? Math.round((program.completed_days / program.days_count) * 100) : 0}% complete
														</span>
													</div>
												</div>
												<Icon icon="ph:arrow-right-bold" width="20" class="text-gray-400" />
											</div>
										</button>
									{/each}
								</div>
							{/if}
						</div>
					</Card>
				</div>

				<div>
					<Card>
						<div class="p-6">
							<h2 class="mb-4 text-lg font-semibold text-white">Quick Actions</h2>
							<div class="space-y-2">
								<button
									onclick={() => goto('/programs')}
									class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
								>
									<Icon icon="ph:folder-bold" width="20" class="text-blue-500" />
									<span class="text-sm text-white">View Programs</span>
								</button>
								<button
									onclick={() => goto('/analytics')}
									class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
								>
									<Icon icon="ph:chart-bar-bold" width="20" class="text-green-500" />
									<span class="text-sm text-white">Analytics</span>
								</button>
								<button
									onclick={() => goto('/search')}
									class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
								>
									<Icon icon="ph:magnifying-glass-bold" width="20" class="text-purple-500" />
									<span class="text-sm text-white">Search</span>
								</button>
								<button
									onclick={() => goto('/settings')}
									class="flex w-full items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3 text-left transition-colors hover:border-gray-600 hover:bg-gray-750"
								>
									<Icon icon="ph:gear-bold" width="20" class="text-gray-500" />
									<span class="text-sm text-white">Settings</span>
								</button>
							</div>
						</div>
					</Card>
				</div>
			</div>

			{#if dueReviews.length > 0}
				<Card class="mb-8">
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Due for Review</h2>
							<Badge variant="warning">{dueReviews.length} due</Badge>
						</div>
						<div class="space-y-2">
							{#each dueReviews.slice(0, 5) as review (review.id)}
								<div class="flex items-center justify-between rounded-lg border border-yellow-500/20 bg-yellow-500/5 p-3">
									<div class="flex items-center gap-3">
										<Icon icon="ph:brain-bold" width="20" class="text-yellow-500" />
										<div>
											<p class="text-sm font-medium text-white">Day {review.day_number}: {review.day_title}</p>
											<p class="text-xs text-gray-400">{review.concept_name} • Due: {new Date(review.next_review_date).toLocaleDateString()}</p>
										</div>
									</div>
									<Badge variant="warning">Review</Badge>
								</div>
							{/each}
						</div>
					</div>
				</Card>
			{/if}

			{#if badges.length > 0}
				<Card class="mb-8">
					<div class="p-6">
						<h2 class="mb-4 text-lg font-semibold text-white">Badges Earned</h2>
						<div class="flex flex-wrap gap-3">
							{#each badges as badge (badge.id)}
								<div class="flex items-center gap-2 rounded-lg border border-yellow-500/20 bg-yellow-500/5 px-3 py-2">
									<Icon icon="ph:medal-bold" width="20" class="text-yellow-500" />
									<div>
										<p class="text-sm font-medium text-white">{badge.title}</p>
										<p class="text-xs text-gray-400">{badge.description}</p>
									</div>
								</div>
							{/each}
						</div>
					</div>
				</Card>
			{/if}
		{/if}
	</div>
</div>
