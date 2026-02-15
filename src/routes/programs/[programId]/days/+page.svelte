<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { listDayPlansByModule, createDayPlan, deleteDayPlan, duplicateDayPlan } from '$lib/commands';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Icon from '@iconify/svelte';
	import type { DayPlanSummary } from '$lib/types';

	const moduleId = $derived($page.params.programId);
	let dayPlans = $state<DayPlanSummary[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);

	onMount(async () => {
		await loadDayPlans();
	});

	async function loadDayPlans() {
		isLoading = true;
		error = null;
		try {
			if (!moduleId) {
				error = 'Module ID is required';
				return;
			}
			dayPlans = await listDayPlansByModule(moduleId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load day plans';
			console.error('Error loading day plans:', err);
		} finally {
			isLoading = false;
		}
	}

	function handleCreateDay() {
		goto(`/programs/${moduleId}/days/new`);
	}

	function handleEditDay(dayId: string) {
		goto(`/programs/${moduleId}/days/${dayId}/edit`);
	}

	async function handleDuplicateDay(dayId: string) {
		try {
			await duplicateDayPlan(dayId);
			await loadDayPlans();
		} catch (err) {
			alert(`Failed to duplicate day plan: ${err}`);
		}
	}

	async function handleDeleteDay(dayId: string, dayTitle: string) {
		if (!confirm(`Delete "${dayTitle}"? This cannot be undone.`)) return;
		
		try {
			await deleteDayPlan(dayId);
			await loadDayPlans();
		} catch (err) {
			alert(`Failed to delete day plan: ${err}`);
		}
	}

	function getStatusColor(status: string) {
		switch (status) {
			case 'not_started': return 'bg-gray-500';
			case 'in_progress': return 'bg-blue-500';
			case 'passed': return 'bg-green-500';
			case 'mastery': return 'bg-purple-500';
			case 'blocked': return 'bg-red-500';
			default: return 'bg-gray-500';
		}
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-6 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Day Plans</h1>
				<p class="mt-1 text-gray-400">Manage your daily learning missions</p>
			</div>
			<Button onclick={handleCreateDay} icon="ph:plus-bold">
				Create Day Plan
			</Button>
		</div>

		{#if isLoading}
			<div class="flex h-96 items-center justify-center">
				<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
			</div>
		{:else if error}
			<Card>
				<div class="p-6 text-center">
					<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
					<p class="mt-3 text-red-400">{error}</p>
					<Button onclick={loadDayPlans} variant="outline" class="mt-4">
						Try Again
					</Button>
				</div>
			</Card>
		{:else if dayPlans.length === 0}
			<div class="flex flex-col items-center justify-center py-16">
				<Icon icon="ph:calendar-blank-bold" width="64" class="text-gray-600" />
				<h3 class="mt-4 text-lg font-semibold text-white">No day plans yet</h3>
				<p class="mt-2 text-sm text-gray-400">Create your first day plan to start building your learning curriculum.</p>
				<Button onclick={handleCreateDay} class="mt-6" icon="ph:plus-bold">
					Create Day Plan
				</Button>
			</div>
		{:else}
			<div class="grid gap-4">
				{#each dayPlans as day (day.id)}
					<Card>
						<div class="flex items-start justify-between p-6">
							<div class="flex-1">
								<div class="flex items-center gap-3">
									<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-500/10">
										<span class="text-lg font-bold text-blue-500">{day.day_number}</span>
									</div>
									<div>
										<h3 class="text-lg font-semibold text-white">{day.title}</h3>
										<div class="mt-1 flex items-center gap-2">
											<Badge>{day.estimated_minutes} min</Badge>
										</div>
									</div>
								</div>
								<div class="mt-4 flex flex-wrap gap-2">
									{#if day.checklist_count > 0}
										<Badge variant="info">
											<Icon icon="ph:check-square-bold" width="14" class="mr-1" />
											{day.checklist_count} tasks
										</Badge>
									{/if}
									{#if day.quiz_count > 0}
										<Badge variant="warning">
											<Icon icon="ph:question-bold" width="14" class="mr-1" />
											{day.quiz_count} questions
										</Badge>
									{/if}
									{#if day.tag_count > 0}
										<Badge variant="purple">
											<Icon icon="ph:tag-bold" width="14" class="mr-1" />
											{day.tag_count} concepts
										</Badge>
									{/if}
								</div>
							</div>

							<div class="flex gap-2">
								<Button
									variant="ghost"
									size="sm"
									onclick={() => handleEditDay(day.id)}
									icon="ph:pencil-bold"
								>
									Edit
								</Button>
								<Button
									variant="ghost"
									size="sm"
									onclick={() => handleDuplicateDay(day.id)}
									icon="ph:copy-bold"
								>
									Duplicate
								</Button>
								<Button
									variant="ghost"
									size="sm"
									onclick={() => handleDeleteDay(day.id, day.title)}
									icon="ph:trash-bold"
								>
									Delete
								</Button>
							</div>
						</div>
					</Card>
				{/each}
			</div>
		{/if}
	</div>
</div>
