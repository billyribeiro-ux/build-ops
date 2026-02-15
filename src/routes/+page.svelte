<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { listPrograms, getStreak } from '$lib/commands';
	import type { ProgramSummary, StreakInfo } from '$lib/types';

	let programs = $state<ProgramSummary[]>([]);
	let streak = $state<StreakInfo | null>(null);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let activeDays = $state(3);

	onMount(async () => {
		await loadDashboardData();
	});

	async function loadDashboard() {
		isLoading = true;
		try {
			programs = await listPrograms();
		} catch (err) {
			console.error('Failed to load dashboard:', err);
		} finally {
			isLoading = false;
		}
	}

	function getProgressColor(progress: number) {
		if (progress >= 80) return 'text-green-400';
		if (progress >= 50) return 'text-blue-400';
		return 'text-yellow-400';
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
						<div class="mb-4 flex items-center justify-between">
							<h3 class="text-sm font-medium text-gray-400">Current Streak</h3>
							<Icon icon="ph:fire-bold" width="20" class="text-orange-500" />
						</div>
						<p class="text-4xl font-bold text-white">{streak?.current_streak ?? 0}</p>
						<p class="mt-1 text-sm text-gray-400">days in a row</p>
						{#if streak && streak.freeze_days_remaining > 0}
							<div class="mt-2 flex items-center gap-1 text-xs text-blue-400">
								<Icon icon="ph:snowflake-bold" width="12" />
								<span>{streak.freeze_days_remaining} freezes left</span>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm text-gray-400">Days Completed</p>
								<p class="mt-1 text-3xl font-bold text-blue-400">{totalDaysCompleted}</p>
								<p class="mt-1 text-xs text-gray-500">total</p>
							</div>
							<div class="rounded-full bg-blue-500/10 p-3">
								<Icon icon="ph:check-circle-bold" width="24" class="text-blue-500" />
							</div>
						</div>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm text-gray-400">Average Score</p>
								<p class="mt-1 text-3xl font-bold {getScoreColor(averageScore)}">{averageScore}</p>
								<p class="mt-1 text-xs text-gray-500">out of 100</p>
							</div>
							<div class="rounded-full bg-green-500/10 p-3">
								<Icon icon="ph:chart-line-up-bold" width="24" class="text-green-500" />
							</div>
						</div>
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="flex items-center justify-between">
							<div>
								<p class="text-sm text-gray-400">Active Days</p>
								<p class="mt-1 text-3xl font-bold text-purple-400">{activeDays}</p>
								<p class="mt-1 text-xs text-gray-500">in progress</p>
							</div>
							<div class="rounded-full bg-purple-500/10 p-3">
								<Icon icon="ph:clock-bold" width="24" class="text-purple-500" />
							</div>
						</div>
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

			<Card>
				<div class="p-6">
					<h2 class="mb-4 text-lg font-semibold text-white">Recent Activity</h2>
					<div class="space-y-3">
						<div class="flex items-center gap-4 rounded-lg border border-gray-700 bg-gray-800 p-3">
							<div class="rounded-full bg-green-500/10 p-2">
								<Icon icon="ph:check-bold" width="16" class="text-green-500" />
							</div>
							<div class="flex-1">
								<p class="text-sm text-white">Completed Day 15: Advanced React Patterns</p>
								<p class="text-xs text-gray-400">2 hours ago • Score: 92/100</p>
							</div>
						</div>
						<div class="flex items-center gap-4 rounded-lg border border-gray-700 bg-gray-800 p-3">
							<div class="rounded-full bg-blue-500/10 p-2">
								<Icon icon="ph:play-bold" width="16" class="text-blue-500" />
							</div>
							<div class="flex-1">
								<p class="text-sm text-white">Started Day 16: State Management</p>
								<p class="text-xs text-gray-400">5 hours ago</p>
							</div>
						</div>
						<div class="flex items-center gap-4 rounded-lg border border-gray-700 bg-gray-800 p-3">
							<div class="rounded-full bg-orange-500/10 p-2">
								<Icon icon="ph:fire-bold" width="16" class="text-orange-500" />
							</div>
							<div class="flex-1">
								<p class="text-sm text-white">7-day streak milestone reached!</p>
								<p class="text-xs text-gray-400">Yesterday</p>
							</div>
						</div>
					</div>
				</div>
			</Card>
		{/if}
	</div>
</div>
