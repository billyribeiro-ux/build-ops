<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { 
		getDayPlan, 
		updateDayPlan,
		addChecklistItem,
		updateChecklistItem,
		deleteChecklistItem,
		addQuizQuestion,
		updateQuizQuestion,
		deleteQuizQuestion,
		listConceptTags,
		addTagToDay,
		removeTagFromDay
	} from '$lib/commands';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import type { DayPlanFull, UpdateDayPlanInput, ChecklistItem, QuizQuestion, ConceptTag } from '$lib/types';

	const programId = $derived($page.params.programId);
	const dayId = $derived($page.params.dayId);
	
	let dayPlan = $state<DayPlanFull | null>(null);
	let allTags = $state<ConceptTag[]>([]);
	let isLoading = $state(true);
	let isSaving = $state(false);
	let error = $state<string | null>(null);
	let activeTab = $state<'details' | 'checklist' | 'quiz' | 'tags'>('details');

	let title = $state('');
	let syntaxTargets = $state('');
	let implementationBrief = $state('');
	let minMinutes = $state(30);
	let targetMinutes = $state(60);
	let maxMinutes = $state(90);
	let complexityLevel = $state(3);

	let newChecklistText = $state('');
	let newQuizQuestion = $state('');
	let newQuizAnswer = $state('');

	onMount(async () => {
		await loadDayPlan();
		await loadTags();
	});

	async function loadDayPlan() {
		isLoading = true;
		error = null;
		try {
			if (!dayId) {
				error = 'Day ID is required';
				return;
			}
			dayPlan = await getDayPlan(dayId);
			
			title = dayPlan.day_plan.title;
			syntaxTargets = dayPlan.day_plan.syntax_targets;
			implementationBrief = dayPlan.day_plan.implementation_brief;
			minMinutes = dayPlan.day_plan.min_minutes;
			targetMinutes = dayPlan.day_plan.recommended_minutes;
			maxMinutes = dayPlan.day_plan.deep_minutes;
			complexityLevel = dayPlan.day_plan.complexity_level;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load day plan';
			console.error('Error loading day plan:', err);
		} finally {
			isLoading = false;
		}
	}

	async function loadTags() {
		try {
			allTags = await listConceptTags();
		} catch (err) {
			console.error('Error loading tags:', err);
		}
	}

	async function handleSave() {
		if (!dayId) return;

		isSaving = true;
		error = null;

		try {
			const input: UpdateDayPlanInput = {
				title,
				syntax_targets: syntaxTargets,
				implementation_brief: implementationBrief
			};

			await updateDayPlan(dayId, input);
			await loadDayPlan();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to save day plan';
			console.error('Error saving day plan:', err);
		} finally {
			isSaving = false;
		}
	}

	async function handleAddChecklistItem() {
		if (!dayId || !newChecklistText.trim()) return;

		try {
			await addChecklistItem({
				day_plan_id: dayId,
				label: newChecklistText,
				is_required: false
			});
			newChecklistText = '';
			await loadDayPlan();
		} catch (err) {
			alert(`Failed to add checklist item: ${err}`);
		}
	}

	async function handleDeleteChecklistItem(itemId: string) {
		try {
			await deleteChecklistItem(itemId);
			await loadDayPlan();
		} catch (err) {
			alert(`Failed to delete checklist item: ${err}`);
		}
	}

	async function handleAddQuizQuestion() {
		if (!dayId || !newQuizQuestion.trim() || !newQuizAnswer.trim()) return;

		try {
			await addQuizQuestion({
				day_plan_id: dayId,
				question_text: newQuizQuestion,
				question_type: 'short_answer',
				correct_answer: newQuizAnswer,
				points: 10,
				time_limit_seconds: 300
			});
			newQuizQuestion = '';
			newQuizAnswer = '';
			await loadDayPlan();
		} catch (err) {
			alert(`Failed to add quiz question: ${err}`);
		}
	}

	async function handleDeleteQuizQuestion(questionId: string) {
		try {
			await deleteQuizQuestion(questionId);
			await loadDayPlan();
		} catch (err) {
			alert(`Failed to delete quiz question: ${err}`);
		}
	}

	async function handleToggleTag(tagId: string) {
		if (!dayId) return;

		const isTagged = dayPlan?.concept_tags.some(t => t.id === tagId);
		
		try {
			if (isTagged) {
				await removeTagFromDay(dayId, tagId);
			} else {
				await addTagToDay(dayId, tagId);
			}
			await loadDayPlan();
		} catch (err) {
			alert(`Failed to toggle tag: ${err}`);
		}
	}

	function handleBack() {
		goto(`/programs/${programId}/days`);
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-6xl">
		<div class="mb-6">
			<Button variant="ghost" onclick={handleBack} icon="ph:arrow-left-bold">
				Back to Day Plans
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
					<Button onclick={loadDayPlan} variant="outline" class="mt-4">
						Try Again
					</Button>
				</div>
			</Card>
		{:else if dayPlan}
			<div class="mb-6 flex items-center justify-between">
				<div>
					<h1 class="text-2xl font-bold text-white">{dayPlan.day_plan.title}</h1>
					<p class="mt-1 text-gray-400">Day {dayPlan.day_plan.day_number}</p>
				</div>
				<Button onclick={handleSave} disabled={isSaving} icon="ph:floppy-disk-bold">
					{isSaving ? 'Saving...' : 'Save Changes'}
				</Button>
			</div>

			<div class="mb-6 flex gap-2 border-b border-gray-800">
				<button
					onclick={() => activeTab = 'details'}
					class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'details' ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-400 hover:text-white'}"
				>
					Details
				</button>
				<button
					onclick={() => activeTab = 'checklist'}
					class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'checklist' ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-400 hover:text-white'}"
				>
					Checklist ({dayPlan.checklist_items.length})
				</button>
				<button
					onclick={() => activeTab = 'quiz'}
					class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'quiz' ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-400 hover:text-white'}"
				>
					Quiz ({dayPlan.quiz_questions.length})
				</button>
				<button
					onclick={() => activeTab = 'tags'}
					class="px-4 py-2 text-sm font-medium transition-colors {activeTab === 'tags' ? 'border-b-2 border-blue-500 text-blue-500' : 'text-gray-400 hover:text-white'}"
				>
					Concept Tags ({dayPlan.concept_tags.length})
				</button>
			</div>

			{#if activeTab === 'details'}
				<Card>
					<div class="p-6 space-y-6">
						<div class="grid grid-cols-2 gap-4">
							<div class="col-span-2">
								<label for="title" class="block text-sm font-medium text-gray-300">Title</label>
								<input
									id="title"
									type="text"
									bind:value={title}
									class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								/>
							</div>

							<div>
								<label for="complexity" class="block text-sm font-medium text-gray-300">
									Complexity Level
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

							<div class="grid grid-cols-3 gap-2">
								<div>
									<label for="minMinutes" class="block text-sm font-medium text-gray-300">Min</label>
									<input
										id="minMinutes"
										type="number"
										bind:value={minMinutes}
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label for="targetMinutes" class="block text-sm font-medium text-gray-300">Target</label>
									<input
										id="targetMinutes"
										type="number"
										bind:value={targetMinutes}
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label for="maxMinutes" class="block text-sm font-medium text-gray-300">Max</label>
									<input
										id="maxMinutes"
										type="number"
										bind:value={maxMinutes}
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
							</div>

							<div class="col-span-2">
								<label for="syntaxTargets" class="block text-sm font-medium text-gray-300">
									Syntax Targets
								</label>
								<textarea
									id="syntaxTargets"
									bind:value={syntaxTargets}
									rows="3"
									class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								></textarea>
							</div>

							<div class="col-span-2">
								<label for="implementationBrief" class="block text-sm font-medium text-gray-300">
									Implementation Brief
								</label>
								<textarea
									id="implementationBrief"
									bind:value={implementationBrief}
									rows="6"
									class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								></textarea>
							</div>
						</div>
					</div>
				</Card>
			{:else if activeTab === 'checklist'}
				<Card>
					<div class="p-6">
						<div class="mb-4 flex gap-2">
							<input
								type="text"
								bind:value={newChecklistText}
								placeholder="Add a checklist item..."
								class="flex-1 rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								onkeydown={(e) => e.key === 'Enter' && handleAddChecklistItem()}
							/>
							<Button onclick={handleAddChecklistItem} icon="ph:plus-bold">
								Add
							</Button>
						</div>

						<div class="space-y-2">
							{#each dayPlan.checklist_items as item (item.id)}
								<div class="flex items-center gap-3 rounded-lg border border-gray-700 bg-gray-800 p-3">
									<Icon icon="ph:check-square-bold" width="20" class="text-gray-400" />
									<span class="flex-1 text-white">{item.label}</span>
									<Button
										variant="ghost"
										size="sm"
										onclick={() => handleDeleteChecklistItem(item.id)}
										icon="ph:trash-bold"
									>
										Delete
									</Button>
								</div>
							{/each}
						</div>
					</div>
				</Card>
			{:else if activeTab === 'quiz'}
				<Card>
					<div class="p-6">
						<div class="mb-4 space-y-2">
							<input
								type="text"
								bind:value={newQuizQuestion}
								placeholder="Quiz question..."
								class="w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
							/>
							<div class="flex gap-2">
								<input
									type="text"
									bind:value={newQuizAnswer}
									placeholder="Correct answer..."
									class="flex-1 rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white placeholder-gray-500 focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
								/>
								<Button onclick={handleAddQuizQuestion} icon="ph:plus-bold">
									Add Question
								</Button>
							</div>
						</div>

						<div class="space-y-3">
							{#each dayPlan.quiz_questions as question (question.id)}
								<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
									<div class="flex items-start justify-between">
										<div class="flex-1">
											<p class="font-medium text-white">{question.question_text}</p>
											<p class="mt-1 text-sm text-green-400">✓ {question.correct_answer}</p>
										</div>
										<Button
											variant="ghost"
											size="sm"
											onclick={() => handleDeleteQuizQuestion(question.id)}
											icon="ph:trash-bold"
										>
											Delete
										</Button>
									</div>
								</div>
							{/each}
						</div>
					</div>
				</Card>
			{:else if activeTab === 'tags'}
				<Card>
					<div class="p-6">
						<h3 class="mb-4 text-lg font-semibold text-white">Concept Tags</h3>
						<div class="flex flex-wrap gap-2">
							{#each allTags as tag (tag.id)}
								{@const isSelected = dayPlan.concept_tags.some(t => t.id === tag.id)}
								<button
									onclick={() => handleToggleTag(tag.id)}
									class="rounded-lg border px-3 py-1.5 text-sm font-medium transition-colors {isSelected ? 'border-blue-500 bg-blue-500/10 text-blue-400' : 'border-gray-700 bg-gray-800 text-gray-400 hover:border-gray-600'}"
								>
									{tag.name}
								</button>
							{/each}
						</div>
					</div>
				</Card>
			{/if}
		{/if}
	</div>
</div>
