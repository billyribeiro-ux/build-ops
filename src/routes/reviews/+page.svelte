<script lang="ts">
	import { onMount } from 'svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { listPrograms, listAttempts } from '$lib/commands';
	import type { ProgramSummary, DayAttemptSummary } from '$lib/types';

	let programs = $state<ProgramSummary[]>([]);
	let allAttempts = $state<DayAttemptSummary[]>([]);
	let isLoading = $state(true);

	interface WeeklyReview {
		weekLabel: string;
		startDate: Date;
		endDate: Date;
		attempts: DayAttemptSummary[];
		avgScore: number;
		totalMinutes: number;
		passed: number;
		mastery: number;
	}

	let reviews = $state<WeeklyReview[]>([]);

	onMount(async () => {
		try {
			programs = await listPrograms();
			const results = await Promise.all(
				programs.map((p) => listAttempts(p.id).catch(() => []))
			);
			allAttempts = results.flat();
			reviews = generateWeeklyReviews(allAttempts);
		} catch (err) {
			console.error('Failed to load reviews:', err);
		} finally {
			isLoading = false;
		}
	});

	function generateWeeklyReviews(attempts: DayAttemptSummary[]): WeeklyReview[] {
		if (attempts.length === 0) return [];

		const sorted = [...attempts].sort(
			(a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
		);

		const weeks: Map<string, DayAttemptSummary[]> = new Map();
		for (const attempt of sorted) {
			const date = new Date(attempt.created_at);
			const weekStart = new Date(date);
			weekStart.setDate(date.getDate() - date.getDay());
			weekStart.setHours(0, 0, 0, 0);
			const key = weekStart.toISOString();
			if (!weeks.has(key)) weeks.set(key, []);
			weeks.get(key)!.push(attempt);
		}

		return Array.from(weeks.entries())
			.map(([key, weekAttempts]) => {
				const startDate = new Date(key);
				const endDate = new Date(startDate);
				endDate.setDate(startDate.getDate() + 6);
				const scores = weekAttempts.filter((a) => a.total_score > 0);
				return {
					weekLabel: `${startDate.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} - ${endDate.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}`,
					startDate,
					endDate,
					attempts: weekAttempts,
					avgScore:
						scores.length > 0
							? Math.round(scores.reduce((s, a) => s + a.total_score, 0) / scores.length)
							: 0,
					totalMinutes: weekAttempts.reduce((s, a) => s + a.actual_minutes, 0),
					passed: weekAttempts.filter((a) => a.status === 'passed' || a.status === 'mastery').length,
					mastery: weekAttempts.filter((a) => a.status === 'mastery').length
				};
			})
			.reverse();
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-white">Weekly Reviews</h1>
			<p class="mt-1 text-gray-400">Auto-generated summaries of your weekly progress</p>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else if reviews.length === 0}
			<Card>
				<div class="p-12 text-center">
					<Icon icon="ph:calendar-blank-bold" width="48" class="mx-auto text-gray-600" />
					<p class="mt-3 text-gray-400">No weekly reviews yet</p>
					<p class="mt-1 text-sm text-gray-500">Complete some day attempts to generate weekly reviews</p>
				</div>
			</Card>
		{:else}
			<div class="space-y-6">
				{#each reviews as review, i}
					<Card>
						<div class="p-6">
							<div class="mb-4 flex items-center justify-between">
								<div class="flex items-center gap-3">
									<Icon icon="ph:calendar-bold" width="24" class="text-blue-500" />
									<h2 class="text-lg font-semibold text-white">{review.weekLabel}</h2>
									{#if i === 0}
										<Badge variant="info">Current</Badge>
									{/if}
								</div>
								<div class="flex items-center gap-4">
									<Badge variant={review.avgScore >= 95 ? 'purple' : review.avgScore >= 70 ? 'success' : 'warning'}>
										Avg: {review.avgScore}/100
									</Badge>
								</div>
							</div>

							<div class="grid grid-cols-4 gap-4">
								<div class="rounded-lg bg-gray-800 p-4 text-center">
									<p class="text-2xl font-bold text-white">{review.attempts.length}</p>
									<p class="text-xs text-gray-400">Attempts</p>
								</div>
								<div class="rounded-lg bg-gray-800 p-4 text-center">
									<p class="text-2xl font-bold text-green-400">{review.passed}</p>
									<p class="text-xs text-gray-400">Passed</p>
								</div>
								<div class="rounded-lg bg-gray-800 p-4 text-center">
									<p class="text-2xl font-bold text-purple-400">{review.mastery}</p>
									<p class="text-xs text-gray-400">Mastery</p>
								</div>
								<div class="rounded-lg bg-gray-800 p-4 text-center">
									<p class="text-2xl font-bold text-blue-400">{Math.round(review.totalMinutes / 60)}h {review.totalMinutes % 60}m</p>
									<p class="text-xs text-gray-400">Time Spent</p>
								</div>
							</div>

							{#if review.attempts.length > 0}
								<div class="mt-4">
									<h3 class="mb-2 text-sm font-medium text-gray-400">Attempts This Week</h3>
									<div class="space-y-2">
										{#each review.attempts as attempt}
											<div class="flex items-center justify-between rounded-lg border border-gray-700 bg-gray-800/50 px-4 py-2">
												<div class="flex items-center gap-3">
													<span class="text-sm text-white">Day {attempt.day_number}: {attempt.day_title}</span>
													<Badge variant={attempt.status === 'mastery' ? 'purple' : attempt.status === 'passed' ? 'success' : attempt.status === 'in_progress' ? 'info' : 'warning'}>
														{attempt.status}
													</Badge>
												</div>
												<span class="text-sm font-medium text-gray-300">{attempt.total_score}/100</span>
											</div>
										{/each}
									</div>
								</div>
							{/if}
						</div>
					</Card>
				{/each}
			</div>
		{/if}
	</div>
</div>
