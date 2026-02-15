<script lang="ts">
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import { createProgram } from '$lib/commands/programs';
	import { toasts } from '$lib/stores/app.svelte';
	import type { CreateProgramInput } from '$lib/types';

	let title = $state('');
	let description = $state('');
	let targetDays = $state(40);
	let isSubmitting = $state(false);
	let errors = $state<Record<string, string>>({});

	function validate(): boolean {
		errors = {};
		
		if (!title.trim()) {
			errors.title = 'Title is required';
		}
		
		if (targetDays < 1) {
			errors.targetDays = 'Target days must be at least 1';
		}
		
		if (targetDays > 365) {
			errors.targetDays = 'Target days cannot exceed 365';
		}
		
		return Object.keys(errors).length === 0;
	}

	async function handleSubmit() {
		if (!validate()) return;
		
		try {
			isSubmitting = true;
			const input: CreateProgramInput = {
				title: title.trim(),
				description: description.trim(),
				target_days: targetDays
			};
			
			const program = await createProgram(input);
			toasts.success('Program created');
			goto(`/programs/${program.id}`);
		} catch (error) {
			toasts.error(`Failed to create program: ${error}`);
		} finally {
			isSubmitting = false;
		}
	}
</script>

<div class="mx-auto max-w-2xl space-y-6">
	<div class="flex items-center gap-4">
		<button
			onclick={() => goto('/programs')}
			class="rounded-lg p-2 text-text-tertiary transition-colors hover:bg-bg-hover hover:text-text-primary"
		>
			<Icon icon="ph:arrow-left-bold" width="20" />
		</button>
		<div>
			<h1 class="text-2xl font-bold text-text-primary">Create Program</h1>
			<p class="mt-1 text-text-secondary">Set up a new learning program</p>
		</div>
	</div>

	<Card>
		<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
			<div>
				<label for="title" class="mb-2 block text-sm font-medium text-text-primary">
					Title <span class="text-accent-danger">*</span>
				</label>
				<Input
					id="title"
					bind:value={title}
					placeholder="e.g., SvelteKit Mastery"
					error={errors.title}
					disabled={isSubmitting}
				/>
			</div>

			<div>
				<label for="description" class="mb-2 block text-sm font-medium text-text-primary">
					Description
				</label>
				<Textarea
					id="description"
					bind:value={description}
					placeholder="What will you learn in this program?"
					rows={4}
					disabled={isSubmitting}
				/>
			</div>

			<div>
				<label for="targetDays" class="mb-2 block text-sm font-medium text-text-primary">
					Target Days <span class="text-accent-danger">*</span>
				</label>
				<Input
					id="targetDays"
					type="number"
					bind:value={targetDays}
					min={1}
					max={365}
					error={errors.targetDays}
					disabled={isSubmitting}
				/>
				<p class="mt-1 text-xs text-text-tertiary">
					How many days do you plan to spend on this program?
				</p>
			</div>

			<div class="flex gap-3">
				<Button
					type="button"
					variant="outline"
					onclick={() => goto('/programs')}
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
					{isSubmitting ? 'Creating...' : 'Create Program'}
				</Button>
			</div>
		</form>
	</Card>
</div>
