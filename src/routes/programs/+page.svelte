<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import { deleteProgram, duplicateProgram } from '$lib/commands/programs';
	import { toasts } from '$lib/stores/app.svelte';
	import type { ProgramSummary } from '$lib/types';

	let { data } = $props();
	let programs = $state<ProgramSummary[]>(data.programs);
	let confirmDelete = $state(false);
	let programToDelete = $state<string | null>(null);
	let isDeleting = $state(false);

	async function handleDelete(id: string) {
		programToDelete = id;
		confirmDelete = true;
	}

	async function confirmDeleteProgram() {
		if (!programToDelete) return;
		
		try {
			isDeleting = true;
			await deleteProgram(programToDelete);
			programs = programs.filter(p => p.id !== programToDelete);
			toasts.success('Program deleted');
		} catch (error) {
			toasts.error(`Failed to delete program: ${error}`);
		} finally {
			isDeleting = false;
			programToDelete = null;
		}
	}

	async function handleDuplicate(id: string, title: string) {
		try {
			const newProgram = await duplicateProgram(id, `${title} (Copy)`);
			programs = [newProgram as ProgramSummary, ...programs];
			toasts.success('Program duplicated');
		} catch (error) {
			toasts.error(`Failed to duplicate program: ${error}`);
		}
	}

	function getStatusVariant(status: string): 'success' | 'warning' | 'info' | 'default' {
		switch (status) {
			case 'active': return 'success';
			case 'paused': return 'warning';
			case 'completed': return 'info';
			default: return 'default';
		}
	}

	function getProgressPercent(completed: number, total: number): number {
		return total > 0 ? Math.round((completed / total) * 100) : 0;
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-bold text-text-primary">Programs</h1>
			<p class="mt-1 text-text-secondary">Your learning programs and progress</p>
		</div>
		<Button variant="primary" onclick={() => goto('/programs/new')}>
			<Icon icon="ph:plus-bold" width="16" />
			New Program
		</Button>
	</div>

	{#if programs.length === 0}
		<EmptyState
			icon="ph:books-bold"
			title="No programs yet"
			description="Create your first learning program to get started"
		>
			{#snippet action()}
				<Button variant="primary" onclick={() => goto('/programs/new')}>
					<Icon icon="ph:plus-bold" width="16" />
					Create Program
				</Button>
			{/snippet}
		</EmptyState>
	{:else}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each programs as program (program.id)}
				<Card hover onclick={() => goto(`/programs/${program.id}`)}>
					<div class="space-y-4">
						<div class="flex items-start justify-between">
							<div class="flex-1">
								<h3 class="text-lg font-semibold text-text-primary">{program.title}</h3>
								<Badge variant={getStatusVariant(program.status)} size="small" class="mt-2">
									{program.status}
								</Badge>
							</div>
							<div class="relative">
								<button
									class="rounded-lg p-2 text-text-tertiary transition-colors hover:bg-bg-hover hover:text-text-primary"
									onclick={(e) => {
										e.stopPropagation();
										// TODO: Dropdown menu
									}}
								>
									<Icon icon="ph:dots-three-bold" width="20" />
								</button>
							</div>
						</div>

						{#if program.description}
							<p class="line-clamp-2 text-sm text-text-secondary">{program.description}</p>
						{/if}

						<div class="space-y-2">
							<div class="flex items-center justify-between text-xs text-text-tertiary">
								<span>Progress</span>
								<span>{program.completed_days} / {program.days_count} days</span>
							</div>
							<div class="h-2 overflow-hidden rounded-full bg-bg-hover">
								<div
									class="h-full bg-accent-primary transition-all"
									style="width: {getProgressPercent(program.completed_days, program.days_count)}%"
								></div>
							</div>
						</div>

						<div class="flex items-center gap-4 text-xs text-text-tertiary">
							<div class="flex items-center gap-1">
								<Icon icon="ph:calendar-bold" width="14" />
								<span>{program.target_days} day target</span>
							</div>
							{#if program.latest_score !== null}
								<div class="flex items-center gap-1">
									<Icon icon="ph:chart-line-bold" width="14" />
									<span>Latest: {program.latest_score}pts</span>
								</div>
							{/if}
						</div>

						<div class="flex gap-2 pt-2">
							<Button
								variant="outline"
								size="small"
								onclick={(e) => {
									e.stopPropagation();
									handleDuplicate(program.id, program.title);
								}}
								class="flex-1"
							>
								<Icon icon="ph:copy-bold" width="14" />
								Duplicate
							</Button>
							<Button
								variant="danger"
								size="small"
								onclick={(e) => {
									e.stopPropagation();
									handleDelete(program.id);
								}}
								class="flex-1"
							>
								<Icon icon="ph:trash-bold" width="14" />
								Delete
							</Button>
						</div>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>

<ConfirmDialog
	bind:open={confirmDelete}
	title="Delete Program"
	message="Are you sure you want to delete this program? This will permanently delete all modules, day plans, attempts, and evidence. This action cannot be undone."
	confirmText={isDeleting ? 'Deleting...' : 'Delete'}
	variant="danger"
	onconfirm={confirmDeleteProgram}
/>
