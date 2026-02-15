<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { listAttempts } from '$lib/commands';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Icon from '@iconify/svelte';
	import type { DayAttemptSummary } from '$lib/types';

	let attempts = $state<DayAttemptSummary[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	let programId = $derived($page.params.programId);

	onMount(async () => {
		await loadAttempts();
	});

	async function loadAttempts() {
		isLoading = true;
		error = null;
		try {
			if (!programId) {
				throw new Error('Program ID is required');
			}
			attempts = await listAttempts(programId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load attempts';
			console.error('Error loading attempts:', err);
		} finally {
			isLoading = false;
		}
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'passed': return 'success';
			case 'mastery': return 'purple';
			case 'blocked': return 'danger';
			case 'in_progress': return 'warning';
			default: return 'info';
		}
	}

	function formatDate(dateStr: string) {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatDuration(minutes: number | null) {
		if (!minutes) return 'N/A';
		const hours = Math.floor(minutes / 60);
		const mins = minutes % 60;
		return hours > 0 ? `${hours}h ${mins}m` : `${mins}m`;
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Attempt History</h1>
				<p class="mt-1 text-gray-400">Review all your day attempts and progress</p>
			</div>
			<Button onclick={() => goto(`/programs/${programId}`)} variant="outline" icon="ph:arrow-left-bold">
				Back to Program
			</Button>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else if error}
			<Card>
				<div class="p-12 text-center">
					<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
					<p class="mt-3 text-red-400">{error}</p>
				</div>
			</Card>
		{:else if attempts.length === 0}
			<Card>
				<div class="p-12 text-center">
					<Icon icon="ph:clipboard-text-bold" width="48" class="mx-auto text-gray-600" />
					<h2 class="mt-3 text-xl font-semibold text-white">No attempts yet</h2>
					<p class="mt-1 text-gray-400">Start working on day plans to see your attempt history</p>
					<Button onclick={() => goto(`/programs/${programId}`)} class="mt-6" icon="ph:arrow-right-bold">
						View Day Plans
					</Button>
				</div>
			</Card>
		{:else}
			<div class="space-y-4">
				{#each attempts as attempt (attempt.id)}
					<Card>
						<button
							onclick={() => goto(`/work/${attempt.id}`)}
							class="w-full p-6 text-left transition-colors hover:bg-gray-800/50"
						>
							<div class="flex items-start justify-between">
								<div class="flex-1">
									<div class="flex items-center gap-3">
										<h3 class="text-lg font-semibold text-white">
											Attempt #{attempt.attempt_number}
										</h3>
										<Badge variant={getStatusColor(attempt.status)}>
											{attempt.status}
										</Badge>
									</div>
									
									<div class="mt-3 grid grid-cols-3 gap-4">
										<div>
											<p class="text-xs text-gray-500">Started</p>
											<p class="mt-1 text-sm text-gray-300">
												{formatDate(attempt.created_at)}
											</p>
										</div>
										<div>
											<p class="text-xs text-gray-500">Duration</p>
											<p class="mt-1 text-sm text-gray-300">
												{formatDuration(attempt.actual_minutes)}
											</p>
										</div>
										{#if attempt.total_score !== null}
											<div>
												<p class="text-xs text-gray-500">Score</p>
												<p class="mt-1 text-lg font-bold text-white">
													{attempt.total_score}
													<span class="text-sm text-gray-500">/100</span>
												</p>
											</div>
										{/if}
									</div>

									{#if attempt.status === 'in_progress'}
										<div class="mt-4 flex items-center gap-2 text-sm text-blue-400">
											<Icon icon="ph:clock-bold" width="16" />
											<span>In progress - click to continue</span>
										</div>
									{/if}
								</div>
								
								<Icon icon="ph:arrow-right-bold" width="20" class="text-gray-400" />
							</div>
						</button>
					</Card>
				{/each}
			</div>

			<div class="mt-6 rounded-lg border border-gray-700 bg-gray-800 p-6">
				<h3 class="mb-4 text-lg font-semibold text-white">Statistics</h3>
				<div class="grid grid-cols-4 gap-6">
					<div>
						<p class="text-sm text-gray-400">Total Attempts</p>
						<p class="mt-1 text-2xl font-bold text-white">{attempts.length}</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Completed</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{attempts.filter(a => a.status !== 'in_progress').length}
						</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Passed</p>
						<p class="mt-1 text-2xl font-bold text-green-400">
							{attempts.filter(a => a.status === 'passed' || a.status === 'mastery').length}
						</p>
					</div>
					<div>
						<p class="text-sm text-gray-400">Average Score</p>
						<p class="mt-1 text-2xl font-bold text-white">
							{attempts.filter(a => a.total_score !== null).length > 0
								? Math.round(
									attempts
										.filter(a => a.total_score !== null)
										.reduce((sum, a) => sum + (a.total_score || 0), 0) /
									attempts.filter(a => a.total_score !== null).length
								)
								: 'N/A'}
						</p>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
