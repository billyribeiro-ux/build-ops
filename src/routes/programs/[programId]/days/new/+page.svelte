<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { createDayPlan } from '$lib/commands';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Icon from '@iconify/svelte';
	import type { CreateDayPlanInput } from '$lib/types';

	const programId = $derived($page.params.programId);
	
	let title = $state('');
	let dayNumber = $state(1);
	let syntaxTargets = $state('');
	let implementationBrief = $state('');
	let minMinutes = $state(30);
	let targetMinutes = $state(60);
	let maxMinutes = $state(90);
	let complexityLevel = $state(3);
	let focusBlocks = $state('[]');
	let isSubmitting = $state(false);
	let error = $state<string | null>(null);

	async function handleSubmit() {
		if (!title.trim()) {
			error = 'Title is required';
			return;
		}
		if (!syntaxTargets.trim()) {
			error = 'Syntax targets are required';
			return;
		}
		if (!implementationBrief.trim()) {
			error = 'Implementation brief is required';
			return;
		}

		isSubmitting = true;
		error = null;

		try {
			const input: CreateDayPlanInput = {
				program_id: programId!,
				module_id: programId!,
				title,
				day_number: dayNumber,
				syntax_targets: syntaxTargets,
				implementation_brief: implementationBrief,
				min_minutes: minMinutes,
				recommended_minutes: targetMinutes,
				deep_minutes: maxMinutes,
				complexity_level: complexityLevel
			};

			const dayPlan = await createDayPlan(input);
			goto(`/programs/${programId}/days/${dayPlan.id}/edit`);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to create day plan';
			console.error('Error creating day plan:', err);
		} finally {
			isSubmitting = false;
		}
	}

	function handleCancel() {
		goto(`/programs/${programId}/days`);
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-4xl">
		<div class="mb-6">
			<Button variant="ghost" onclick={handleCancel} icon="ph:arrow-left-bold">
				Back to Day Plans
			</Button>
		</div>

		<Card>
			<div class="p-6">
				<h1 class="text-2xl font-bold text-white">Create New Day Plan</h1>
				<p class="mt-2 text-gray-400">Define a new daily learning mission</p>

				{#if error}
					<div class="mt-4 rounded-lg border border-red-500/30 bg-red-500/10 p-4">
						<div class="flex items-center gap-2">
							<Icon icon="ph:warning-bold" width="20" class="text-red-500" />
							<p class="text-sm text-red-400">{error}</p>
						</div>
					</div>
				{/if}

				<form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="mt-6 space-y-6">
					<div class="grid grid-cols-2 gap-4">
						<div class="col-span-2">
							<label for="title" class="block text-sm font-medium text-gray-300">
								Title <span class="text-red-500">*</span>
							</label>
							<input
								id="title"
								type="text"
								bind:value={title}
								placeholder="e.g., Build a Todo List Component"
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								required
							/>
						</div>

						<div>
							<label for="dayNumber" class="block text-sm font-medium text-gray-300">
								Day Number
							</label>
							<input
								id="dayNumber"
								type="number"
								bind:value={dayNumber}
								min="1"
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>

						<div>
							<label for="complexity" class="block text-sm font-medium text-gray-300">
								Complexity Level (1-5)
							</label>
							<select
								id="complexity"
								bind:value={complexityLevel}
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							>
								<option value={1}>1 - Very Easy</option>
								<option value={2}>2 - Easy</option>
								<option value={3}>3 - Medium</option>
								<option value={4}>4 - Hard</option>
								<option value={5}>5 - Very Hard</option>
							</select>
						</div>

						<div class="col-span-2">
							<label for="syntaxTargets" class="block text-sm font-medium text-gray-300">
								Syntax Targets <span class="text-red-500">*</span>
							</label>
							<textarea
								id="syntaxTargets"
								bind:value={syntaxTargets}
								rows="3"
								placeholder="List the specific syntax, APIs, or patterns to practice..."
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								required
							></textarea>
						</div>

						<div class="col-span-2">
							<label for="implementationBrief" class="block text-sm font-medium text-gray-300">
								Implementation Brief <span class="text-red-500">*</span>
							</label>
							<textarea
								id="implementationBrief"
								bind:value={implementationBrief}
								rows="5"
								placeholder="Describe what needs to be built and the acceptance criteria..."
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								required
							></textarea>
						</div>

						<div>
							<label for="minMinutes" class="block text-sm font-medium text-gray-300">
								Minimum Minutes
							</label>
							<input
								id="minMinutes"
								type="number"
								bind:value={minMinutes}
								min="1"
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>

						<div>
							<label for="targetMinutes" class="block text-sm font-medium text-gray-300">
								Target Minutes
							</label>
							<input
								id="targetMinutes"
								type="number"
								bind:value={targetMinutes}
								min="1"
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>

						<div>
							<label for="maxMinutes" class="block text-sm font-medium text-gray-300">
								Maximum Minutes
							</label>
							<input
								id="maxMinutes"
								type="number"
								bind:value={maxMinutes}
								min="1"
								class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
						</div>
					</div>

					<div class="flex justify-end gap-3 border-t border-gray-700 pt-6">
						<Button type="button" variant="outline" onclick={handleCancel}>
							Cancel
						</Button>
						<Button type="submit" disabled={isSubmitting} icon="ph:check-bold">
							{isSubmitting ? 'Creating...' : 'Create Day Plan'}
						</Button>
					</div>
				</form>
			</div>
		</Card>
	</div>
</div>
