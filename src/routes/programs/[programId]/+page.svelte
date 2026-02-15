<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import Select from '$lib/components/ui/Select.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import ConfirmDialog from '$lib/components/ui/ConfirmDialog.svelte';
	import { createModule, updateModule, deleteModule } from '$lib/commands/modules';
	import { toasts } from '$lib/stores/app.svelte';
	import type { Program, Module, CreateModuleInput, UpdateModuleInput } from '$lib/types';

	let { data } = $props();
	let program = $state<Program>(data.program);
	let modules = $state<Module[]>(data.modules);
	
	let showModuleModal = $state(false);
	let editingModule = $state<Module | null>(null);
	let moduleTitle = $state('');
	let moduleDescription = $state('');
	let moduleColor = $state('#6366F1');
	let isSubmitting = $state(false);
	
	let confirmDelete = $state(false);
	let moduleToDelete = $state<string | null>(null);

	const colorOptions = [
		{ value: '#6366F1', label: 'Indigo' },
		{ value: '#22C55E', label: 'Green' },
		{ value: '#3B82F6', label: 'Blue' },
		{ value: '#F59E0B', label: 'Amber' },
		{ value: '#EF4444', label: 'Red' },
		{ value: '#A855F7', label: 'Purple' },
		{ value: '#EC4899', label: 'Pink' },
		{ value: '#14B8A6', label: 'Teal' }
	];

	function openCreateModule() {
		editingModule = null;
		moduleTitle = '';
		moduleDescription = '';
		moduleColor = '#6366F1';
		showModuleModal = true;
	}

	function openEditModule(module: Module) {
		editingModule = module;
		moduleTitle = module.title;
		moduleDescription = module.description;
		moduleColor = module.color;
		showModuleModal = true;
	}

	async function handleModuleSubmit() {
		if (!moduleTitle.trim()) {
			toasts.error('Module title is required');
			return;
		}

		try {
			isSubmitting = true;
			
			if (editingModule) {
				const input: UpdateModuleInput = {
					title: moduleTitle.trim(),
					description: moduleDescription.trim(),
					color: moduleColor
				};
				const updated = await updateModule(editingModule.id, input);
				modules = modules.map(m => m.id === updated.id ? updated : m);
				toasts.success('Module updated');
			} else {
				const input: CreateModuleInput = {
					program_id: program.id,
					title: moduleTitle.trim(),
					description: moduleDescription.trim(),
					color: moduleColor
				};
				const created = await createModule(input);
				modules = [...modules, created];
				toasts.success('Module created');
			}
			
			showModuleModal = false;
		} catch (error) {
			toasts.error(`Failed to save module: ${error}`);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleDeleteModule(id: string) {
		moduleToDelete = id;
		confirmDelete = true;
	}

	async function confirmDeleteModule() {
		if (!moduleToDelete) return;
		
		try {
			await deleteModule(moduleToDelete);
			modules = modules.filter(m => m.id !== moduleToDelete);
			toasts.success('Module deleted');
		} catch (error) {
			toasts.error(`Failed to delete module: ${error}`);
		} finally {
			moduleToDelete = null;
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
</script>

<div class="space-y-6">
	<div class="flex items-center gap-4">
		<button
			onclick={() => goto('/programs')}
			class="rounded-lg p-2 text-text-tertiary transition-colors hover:bg-bg-hover hover:text-text-primary"
		>
			<Icon icon="ph:arrow-left-bold" width="20" />
		</button>
		<div class="flex-1">
			<h1 class="text-2xl font-bold text-text-primary">{program.title}</h1>
			<p class="mt-1 text-text-secondary">{program.description}</p>
		</div>
		<Badge variant={getStatusVariant(program.status)}>
			{program.status}
		</Badge>
	</div>

	<Card>
		<div class="space-y-4">
			<div class="flex items-center justify-between">
				<h2 class="text-lg font-semibold text-text-primary">Program Details</h2>
			</div>
			
			<div class="grid gap-4 md:grid-cols-3">
				<div>
					<p class="text-sm text-text-tertiary">Target Days</p>
					<p class="mt-1 text-2xl font-bold text-text-primary">{program.target_days}</p>
				</div>
				<div>
					<p class="text-sm text-text-tertiary">Modules</p>
					<p class="mt-1 text-2xl font-bold text-text-primary">{modules.length}</p>
				</div>
				<div>
					<p class="text-sm text-text-tertiary">Created</p>
					<p class="mt-1 text-sm text-text-secondary">
						{new Date(program.created_at).toLocaleDateString()}
					</p>
				</div>
			</div>
		</div>
	</Card>

	<div class="space-y-4">
		<div class="flex items-center justify-between">
			<h2 class="text-lg font-semibold text-text-primary">Modules</h2>
			<Button variant="primary" onclick={openCreateModule}>
				<Icon icon="ph:plus-bold" width="16" />
				Add Module
			</Button>
		</div>

		{#if modules.length === 0}
			<EmptyState
				icon="ph:stack-bold"
				title="No modules yet"
				description="Add your first module to organize your learning days"
			>
				{#snippet action()}
					<Button variant="primary" onclick={openCreateModule}>
						<Icon icon="ph:plus-bold" width="16" />
						Add Module
					</Button>
				{/snippet}
			</EmptyState>
		{:else}
			<div class="space-y-3">
				{#each modules as module (module.id)}
					<Card hover>
						<div class="flex items-start gap-4">
							<div
								class="h-12 w-1 rounded-full"
								style="background-color: {module.color}"
							></div>
							<div class="flex-1">
								<h3 class="font-semibold text-text-primary">{module.title}</h3>
								{#if module.description}
									<p class="mt-1 text-sm text-text-secondary">{module.description}</p>
								{/if}
								<p class="mt-2 text-xs text-text-tertiary">
									Order: {module.order_index + 1}
								</p>
							</div>
							<div class="flex gap-2">
								<Button
									variant="outline"
									size="small"
									onclick={() => openEditModule(module)}
								>
									<Icon icon="ph:pencil-bold" width="14" />
									Edit
								</Button>
								<Button
									variant="danger"
									size="small"
									onclick={() => handleDeleteModule(module.id)}
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
</div>

<Modal bind:open={showModuleModal} title={editingModule ? 'Edit Module' : 'Create Module'}>
	<form onsubmit={(e) => { e.preventDefault(); handleModuleSubmit(); }} class="space-y-4">
		<div>
			<label for="moduleTitle" class="mb-2 block text-sm font-medium text-text-primary">
				Title <span class="text-accent-danger">*</span>
			</label>
			<Input
				id="moduleTitle"
				bind:value={moduleTitle}
				placeholder="e.g., HTML Foundations"
				disabled={isSubmitting}
			/>
		</div>

		<div>
			<label for="moduleDescription" class="mb-2 block text-sm font-medium text-text-primary">
				Description
			</label>
			<Textarea
				id="moduleDescription"
				bind:value={moduleDescription}
				placeholder="What topics does this module cover?"
				rows={3}
				disabled={isSubmitting}
			/>
		</div>

		<div>
			<label for="moduleColor" class="mb-2 block text-sm font-medium text-text-primary">
				Color
			</label>
			<Select
				id="moduleColor"
				bind:value={moduleColor}
				options={colorOptions}
				disabled={isSubmitting}
			/>
		</div>

		<div class="flex gap-3">
			<Button
				type="button"
				variant="outline"
				onclick={() => showModuleModal = false}
				disabled={isSubmitting}
				class="flex-1"
			>
				Cancel
			</Button>
			<Button
				type="submit"
				variant="primary"
				disabled={isSubmitting}
				class="flex-1"
			>
				{isSubmitting ? 'Saving...' : editingModule ? 'Update' : 'Create'}
			</Button>
		</div>
	</form>
</Modal>

<ConfirmDialog
	bind:open={confirmDelete}
	title="Delete Module"
	message="Are you sure you want to delete this module? This will also delete all day plans within this module. This action cannot be undone."
	confirmText="Delete"
	variant="danger"
	onconfirm={confirmDeleteModule}
/>
