<script lang="ts">
	import { onMount } from 'svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { listPrograms, getSkillRadar, listAttempts, listBugLogs } from '$lib/commands';
	import type { ProgramSummary, SkillRadarData, DayAttemptSummary } from '$lib/types';
	import type { BugLog } from '$lib/types/bug-log';

	let selectedTimeRange = $state('7d');
	let programs = $state<ProgramSummary[]>([]);
	let skillData = $state<SkillRadarData[]>([]);
	let allAttempts = $state<DayAttemptSummary[]>([]);
	let allBugs = $state<BugLog[]>([]);
	let isLoading = $state(true);

	let totalTime = $derived(allAttempts.reduce((s, a) => s + a.actual_minutes, 0));
	let avgScore = $derived(
		allAttempts.length > 0
			? Math.round(allAttempts.reduce((s, a) => s + a.total_score, 0) / allAttempts.length)
			: 0
	);
	let bugsByCategory = $derived(
		allBugs.reduce<Record<string, number>>((acc, b) => {
			acc[b.category] = (acc[b.category] || 0) + 1;
			return acc;
		}, {})
	);

	onMount(async () => {
		try {
			programs = await listPrograms();
			if (programs.length > 0) {
				const pid = programs[0].id;
				skillData = await getSkillRadar(pid).catch(() => []);
				const attemptResults = await Promise.all(
					programs.map((p) =>
						listAttempts(p.id).catch(() => [])
					)
				);
				allAttempts = attemptResults.flat();
			}
		} catch (err) {
			console.error('Failed to load analytics:', err);
		} finally {
			isLoading = false;
		}
	});
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Analytics</h1>
				<p class="mt-1 text-gray-400">Comprehensive insights into your learning progress</p>
			</div>
			<div class="flex gap-2">
				<button
					onclick={() => selectedTimeRange = '7d'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === '7d' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					7 Days
				</button>
				<button
					onclick={() => selectedTimeRange = '30d'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === '30d' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					30 Days
				</button>
				<button
					onclick={() => selectedTimeRange = 'all'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === 'all' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					All Time
				</button>
			</div>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else}
			<div class="mb-6 grid grid-cols-4 gap-4">
				<Card>
					<div class="p-4 text-center">
						<p class="text-sm text-gray-400">Total Attempts</p>
						<p class="mt-1 text-2xl font-bold text-white">{allAttempts.length}</p>
					</div>
				</Card>
				<Card>
					<div class="p-4 text-center">
						<p class="text-sm text-gray-400">Avg Score</p>
						<p class="mt-1 text-2xl font-bold text-green-400">{avgScore}/100</p>
					</div>
				</Card>
				<Card>
					<div class="p-4 text-center">
						<p class="text-sm text-gray-400">Total Time</p>
						<p class="mt-1 text-2xl font-bold text-blue-400">{Math.round(totalTime / 60)}h {totalTime % 60}m</p>
					</div>
				</Card>
				<Card>
					<div class="p-4 text-center">
						<p class="text-sm text-gray-400">Bugs Logged</p>
						<p class="mt-1 text-2xl font-bold text-red-400">{allBugs.length}</p>
					</div>
				</Card>
			</div>

			<div class="grid grid-cols-2 gap-6">
				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Skill Radar</h2>
							<Badge variant="info">{skillData.length} domains</Badge>
						</div>
						{#if skillData.length > 0}
							<div class="space-y-3">
								{#each skillData as skill}
									<div>
										<div class="mb-1 flex justify-between text-sm">
											<span class="text-gray-300">{skill.domain}</span>
											<span class="text-white font-medium">{Math.round(skill.score)}/{skill.max_score}</span>
										</div>
										<div class="h-3 overflow-hidden rounded-full bg-gray-700">
											<div
												class="h-full rounded-full bg-blue-500 transition-all"
												style="width: {skill.max_score > 0 ? (skill.score / skill.max_score) * 100 : 0}%"
											></div>
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>Complete attempts to see skill data</p>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Score Trends</h2>
							<Badge variant="success">{allAttempts.length} attempts</Badge>
						</div>
						{#if allAttempts.length > 0}
							<div class="flex h-48 items-end gap-1">
								{#each allAttempts.slice(-20) as attempt}
									<div
										class="flex-1 rounded-t transition-all {attempt.total_score >= 95 ? 'bg-purple-500' : attempt.total_score >= 70 ? 'bg-green-500' : 'bg-yellow-500'}"
										style="height: {Math.max(4, attempt.total_score * 1.8)}px"
										title="Score: {attempt.total_score}"
									></div>
								{/each}
							</div>
							<div class="mt-2 flex justify-between text-xs text-gray-500">
								<span>Oldest</span>
								<span>Latest</span>
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>Complete attempts to see trends</p>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Burndown</h2>
							<Badge variant="purple">{programs.length} programs</Badge>
						</div>
						{#if programs.length > 0}
							<div class="space-y-4">
								{#each programs as program}
									<div>
										<div class="mb-1 flex justify-between text-sm">
											<span class="text-gray-300 truncate">{program.title}</span>
											<span class="text-white">{program.completed_days}/{program.days_count}</span>
										</div>
										<div class="h-3 overflow-hidden rounded-full bg-gray-700">
											<div
												class="h-full rounded-full bg-purple-500 transition-all"
												style="width: {program.days_count > 0 ? (program.completed_days / program.days_count) * 100 : 0}%"
											></div>
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>Create programs to see burndown</p>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Time Distribution</h2>
							<Badge variant="info">{Math.round(totalTime / 60)}h total</Badge>
						</div>
						{#if allAttempts.length > 0}
							<div class="flex h-48 items-end gap-1">
								{#each allAttempts.slice(-20) as attempt}
									<div
										class="flex-1 rounded-t bg-blue-500 transition-all"
										style="height: {Math.max(4, Math.min(180, attempt.actual_minutes * 1.5))}px"
										title="{attempt.actual_minutes} min"
									></div>
								{/each}
							</div>
							<div class="mt-2 flex justify-between text-xs text-gray-500">
								<span>Oldest</span>
								<span>Latest</span>
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>Complete attempts to see time data</p>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Concept Heatmap</h2>
							<Badge variant="warning">Activity</Badge>
						</div>
						{#if allAttempts.length > 0}
							<div class="grid grid-cols-7 gap-1">
								{#each Array(28) as _, i}
									{@const hasActivity = allAttempts.some((a) => {
										const d = new Date(a.created_at);
										const diff = Math.floor((Date.now() - d.getTime()) / 86400000);
										return diff === 27 - i;
									})}
									<div
										class="aspect-square rounded-sm {hasActivity ? 'bg-green-500' : 'bg-gray-700'}"
										title="{27 - i} days ago"
									></div>
								{/each}
							</div>
							<div class="mt-3 flex items-center justify-end gap-2 text-xs text-gray-500">
								<span>Less</span>
								<div class="h-3 w-3 rounded-sm bg-gray-700"></div>
								<div class="h-3 w-3 rounded-sm bg-green-500"></div>
								<span>More</span>
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>Complete attempts to see activity</p>
							</div>
						{/if}
					</div>
				</Card>

				<Card>
					<div class="p-6">
						<div class="mb-4 flex items-center justify-between">
							<h2 class="text-lg font-semibold text-white">Bug Patterns</h2>
							<Badge variant="danger">{allBugs.length} total</Badge>
						</div>
						{#if Object.keys(bugsByCategory).length > 0}
							<div class="space-y-3">
								{#each Object.entries(bugsByCategory) as [category, count]}
									<div>
										<div class="mb-1 flex justify-between text-sm">
											<span class="text-gray-300">{category}</span>
											<span class="text-white">{count}</span>
										</div>
										<div class="h-3 overflow-hidden rounded-full bg-gray-700">
											<div
												class="h-full rounded-full bg-red-500 transition-all"
												style="width: {allBugs.length > 0 ? (count / allBugs.length) * 100 : 0}%"
											></div>
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<div class="flex h-48 items-center justify-center text-gray-500">
								<p>No bugs logged yet</p>
							</div>
						{/if}
					</div>
				</Card>
			</div>
		{/if}
	</div>
</div>
