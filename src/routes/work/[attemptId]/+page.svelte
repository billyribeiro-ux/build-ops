<script lang="ts">
	import { page } from '$app/stores';
	import { onMount, onDestroy } from 'svelte';
	import { 
		getAttempt, 
		updateAttempt,
		autosaveAttempt,
		submitAttempt,
		createExerciseEntry,
		listExerciseEntries,
		createArtifact,
		listArtifacts,
		createBugLog,
		listBugLogs,
		listSessions,
		createSession,
		startSession,
		pauseSession,
		completeSession
	} from '$lib/commands';
	import Button from '$lib/components/ui/Button.svelte';
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import type { DayAttempt, ExerciseEntry, Artifact, BugLog, DaySession, SubmitScoresInput } from '$lib/types';

	const attemptId = $derived($page.params.attemptId);
	
	let attempt = $state<DayAttempt | null>(null);
	let exercises = $state<ExerciseEntry[]>([]);
	let artifacts = $state<Artifact[]>([]);
	let bugs = $state<BugLog[]>([]);
	let sessions = $state<DaySession[]>([]);
	let isLoading = $state(true);
	let error = $state<string | null>(null);
	
	let activeTab = $state<'code' | 'checklist' | 'bugs' | 'evidence' | 'quiz'>('checklist');
	let autosaveInterval: number | null = null;
	
	let implementationScore = $state(0);
	let codeQualityScore = $state(0);
	let accessibilityScore = $state(0);
	let performanceScore = $state(0);
	let quizScore = $state(0);

	onMount(async () => {
		await loadAttempt();
		startAutosave();
	});

	onDestroy(() => {
		if (autosaveInterval) {
			clearInterval(autosaveInterval);
		}
	});

	async function loadAttempt() {
		isLoading = true;
		error = null;
		try {
			if (!attemptId) {
				error = 'Attempt ID is required';
				return;
			}
			attempt = await getAttempt(attemptId);
			
			exercises = await listExerciseEntries(attemptId);
			artifacts = await listArtifacts(attemptId);
			bugs = await listBugLogs(attemptId);
			sessions = await listSessions(attemptId);
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to load attempt';
			console.error('Error loading attempt:', err);
		} finally {
			isLoading = false;
		}
	}

	function startAutosave() {
		autosaveInterval = setInterval(async () => {
			if (attemptId) {
				try {
					await autosaveAttempt(attemptId);
				} catch (err) {
					console.error('Autosave failed:', err);
				}
			}
		}, 30000) as unknown as number;
	}

	async function handleSubmit() {
		if (!attemptId) return;

		const input: SubmitScoresInput = {
			score_implementation: implementationScore,
			score_code_quality: codeQualityScore,
			score_accessibility: accessibilityScore,
			score_performance: performanceScore,
			score_quiz: quizScore,
			daily_summary: '',
			what_went_well: '',
			what_to_improve: '',
			key_learnings: '',
			memory_rebuild_passed: false,
			memory_rebuild_notes: ''
		};

		try {
			await submitAttempt(attemptId, input);
			await loadAttempt();
		} catch (err) {
			alert(`Failed to submit: ${err}`);
		}
	}

	async function handleAddExercise() {
		if (!attemptId) return;
		const title = prompt('Exercise title:');
		if (!title) return;

		try {
			await createExerciseEntry({
				day_attempt_id: attemptId,
				language: 'javascript',
				code: '',
				notes: title
			});
			await loadAttempt();
		} catch (err) {
			alert(`Failed to add exercise: ${err}`);
		}
	}

	async function handleAddArtifact() {
		if (!attemptId) return;
		const title = prompt('Artifact title:');
		if (!title) return;

		try {
			await createArtifact({
				day_attempt_id: attemptId,
				title,
				artifact_type: 'file',
				content: ''
			});
			await loadAttempt();
		} catch (err) {
			alert(`Failed to add artifact: ${err}`);
		}
	}

	async function handleAddBug() {
		if (!attemptId) return;
		const title = prompt('Bug title:');
		if (!title) return;

		try {
			await createBugLog({
				day_attempt_id: attemptId,
				category: 'general',
				severity: 'medium',
				symptom: title,
				root_cause: '',
				fix_applied: '',
				prevention_strategy: '',
				time_to_fix_minutes: 0
			});
			await loadAttempt();
		} catch (err) {
			alert(`Failed to add bug: ${err}`);
		}
	}

	async function handleStartSession() {
		if (!attemptId) return;

		try {
			const session = await createSession({
				day_attempt_id: attemptId,
				session_type: 'build',
				planned_minutes: 60
			});
			await startSession(session.id);
			await loadAttempt();
		} catch (err) {
			alert(`Failed to start session: ${err}`);
		}
	}

	function getTotalScore() {
		return implementationScore + codeQualityScore + accessibilityScore + performanceScore + quizScore;
	}

	function getScoreColor(score: number) {
		if (score >= 95) return 'text-purple-400';
		if (score >= 70) return 'text-green-400';
		return 'text-yellow-400';
	}
</script>

<div class="flex h-screen bg-gray-950">
	{#if isLoading}
		<div class="flex flex-1 items-center justify-center">
			<Icon icon="ph:spinner-bold" width="48" class="animate-spin text-blue-500" />
		</div>
	{:else if error}
		<div class="flex flex-1 items-center justify-center">
			<Card>
				<div class="p-6 text-center">
					<Icon icon="ph:warning-bold" width="48" class="mx-auto text-red-500" />
					<p class="mt-3 text-red-400">{error}</p>
				</div>
			</Card>
		</div>
	{:else if attempt}
		<div class="flex flex-1 flex-col">
			<div class="border-b border-gray-800 bg-gray-900 px-6 py-4">
				<div class="flex items-center justify-between">
					<div>
						<h1 class="text-xl font-bold text-white">Working Session</h1>
						<p class="text-sm text-gray-400">Attempt #{attempt.attempt_number}</p>
					</div>
					<div class="flex items-center gap-4">
						<div class="text-right">
							<div class="text-2xl font-bold {getScoreColor(getTotalScore())}">{getTotalScore()}/100</div>
							<div class="text-xs text-gray-400">Total Score</div>
						</div>
						<Button onclick={handleStartSession} icon="ph:play-bold" variant="outline">
							Start Session
						</Button>
						<Button onclick={handleSubmit} icon="ph:check-bold">
							Submit
						</Button>
					</div>
				</div>
			</div>

			<div class="flex flex-1 overflow-hidden">
				<div class="flex w-16 flex-col border-r border-gray-800 bg-gray-900">
					<button
						onclick={() => activeTab = 'checklist'}
						class="flex h-16 items-center justify-center border-b border-gray-800 transition-colors {activeTab === 'checklist' ? 'bg-blue-500/10 text-blue-500' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
						title="Exercises"
					>
						<Icon icon="ph:list-checks-bold" width="24" />
					</button>
					<button
						onclick={() => activeTab = 'bugs'}
						class="flex h-16 items-center justify-center border-b border-gray-800 transition-colors {activeTab === 'bugs' ? 'bg-blue-500/10 text-blue-500' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
						title="Bug Logs"
					>
						<Icon icon="ph:bug-bold" width="24" />
					</button>
					<button
						onclick={() => activeTab = 'evidence'}
						class="flex h-16 items-center justify-center border-b border-gray-800 transition-colors {activeTab === 'evidence' ? 'bg-blue-500/10 text-blue-500' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
						title="Evidence/Artifacts"
					>
						<Icon icon="ph:image-bold" width="24" />
					</button>
					<button
						onclick={() => activeTab = 'quiz'}
						class="flex h-16 items-center justify-center border-b border-gray-800 transition-colors {activeTab === 'quiz' ? 'bg-blue-500/10 text-blue-500' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
						title="Quiz & Scoring"
					>
						<Icon icon="ph:exam-bold" width="24" />
					</button>
				</div>

				<div class="flex-1 overflow-auto">
					{#if activeTab === 'checklist'}
						<div class="p-6">
							<div class="mb-4 flex items-center justify-between">
								<h2 class="text-lg font-semibold text-white">Exercises ({exercises.length})</h2>
								<Button onclick={handleAddExercise} size="sm" icon="ph:plus-bold">
									Add Exercise
								</Button>
							</div>
							<div class="space-y-3">
								{#each exercises as exercise (exercise.id)}
									<Card>
										<div class="p-4">
											<h3 class="font-medium text-white">{exercise.language}</h3>
											{#if exercise.notes}
												<p class="mt-1 text-sm text-gray-400">{exercise.notes}</p>
											{/if}
											<div class="mt-2">
												<Badge variant="info">{exercise.language}</Badge>
											</div>
										</div>
									</Card>
								{/each}
							</div>
						</div>
					{:else if activeTab === 'bugs'}
						<div class="p-6">
							<div class="mb-4 flex items-center justify-between">
								<h2 class="text-lg font-semibold text-white">Bug Logs ({bugs.length})</h2>
								<Button onclick={handleAddBug} size="sm" icon="ph:plus-bold">
									Add Bug
								</Button>
							</div>
							<div class="space-y-3">
								{#each bugs as bug (bug.id)}
									<Card>
										<div class="p-4">
											<div class="flex items-start justify-between">
												<div class="flex-1">
													<h3 class="font-medium text-white">{bug.symptom}</h3>
													{#if bug.root_cause}
														<p class="mt-1 text-sm text-gray-400">{bug.root_cause}</p>
													{/if}
													{#if bug.fix_applied}
														<pre class="mt-2 rounded bg-green-500/10 p-2 text-xs text-green-400">{bug.fix_applied}</pre>
													{/if}
												</div>
												<Badge variant={bug.severity === 'critical' ? 'danger' : 'warning'}>
													{bug.severity}
												</Badge>
											</div>
										</div>
									</Card>
								{/each}
							</div>
						</div>
					{:else if activeTab === 'evidence'}
						<div class="p-6">
							<div class="mb-4 flex items-center justify-between">
								<h2 class="text-lg font-semibold text-white">Artifacts ({artifacts.length})</h2>
								<Button onclick={handleAddArtifact} size="sm" icon="ph:plus-bold">
									Add Artifact
								</Button>
							</div>
							<div class="grid grid-cols-2 gap-4">
								{#each artifacts as artifact (artifact.id)}
									<Card>
										<div class="p-4">
											<div class="flex items-center gap-2">
												<Icon icon="ph:file-bold" width="20" class="text-blue-500" />
												<h3 class="font-medium text-white">{artifact.title}</h3>
											</div>
											{#if artifact.content}
												<p class="mt-2 text-sm text-gray-400">{artifact.content}</p>
											{/if}
											<Badge variant="info" class="mt-2">{artifact.artifact_type}</Badge>
										</div>
									</Card>
								{/each}
							</div>
						</div>
					{:else if activeTab === 'quiz'}
						<div class="p-6">
							<h2 class="mb-6 text-lg font-semibold text-white">Score Breakdown</h2>
							<div class="space-y-4">
								<div>
									<label class="block text-sm font-medium text-gray-300">
										Implementation (0-40 pts)
									</label>
									<input
										type="number"
										bind:value={implementationScore}
										min="0"
										max="40"
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-300">
										Code Quality (0-20 pts)
									</label>
									<input
										type="number"
										bind:value={codeQualityScore}
										min="0"
										max="20"
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-300">
										Accessibility (0-15 pts)
									</label>
									<input
										type="number"
										bind:value={accessibilityScore}
										min="0"
										max="15"
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-300">
										Performance (0-15 pts)
									</label>
									<input
										type="number"
										bind:value={performanceScore}
										min="0"
										max="15"
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div>
									<label class="block text-sm font-medium text-gray-300">
										Quiz (0-10 pts)
									</label>
									<input
										type="number"
										bind:value={quizScore}
										min="0"
										max="10"
										class="mt-1 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
								</div>
								<div class="border-t border-gray-700 pt-4">
									<div class="flex items-center justify-between">
										<span class="text-lg font-semibold text-white">Total Score</span>
										<span class="text-2xl font-bold {getScoreColor(getTotalScore())}">{getTotalScore()}/100</span>
									</div>
									{#if getTotalScore() >= 95}
										<p class="mt-2 text-sm text-purple-400">🏆 Mastery Level!</p>
									{:else if getTotalScore() >= 70}
										<p class="mt-2 text-sm text-green-400">✓ Passing</p>
									{:else}
										<p class="mt-2 text-sm text-yellow-400">Keep working...</p>
									{/if}
								</div>
							</div>
						</div>
					{/if}
				</div>
			</div>
		</div>
	{/if}
</div>
