<script lang="ts">
	import { timerStore } from '$lib/stores/timer.svelte';
	import { sessionCommands } from '$lib/commands';
	import Icon from '@iconify/svelte';
	import type { DaySession } from '$lib/types';

	let { session }: { session: DaySession } = $props();

	let isActive = $derived(timerStore.activeSession?.id === session.id);
	let formattedTime = $derived(timerStore.formattedTime);
	let elapsedMinutes = $derived(timerStore.elapsedMinutes);
	let isRunning = $derived(timerStore.isRunning);

	async function handleStart() {
		try {
			const updated = await sessionCommands.start(session.id);
			timerStore.start(updated);
		} catch (error) {
			console.error('Failed to start session:', error);
		}
	}

	async function handlePause() {
		try {
			await sessionCommands.pause(session.id);
			timerStore.pause();
		} catch (error) {
			console.error('Failed to pause session:', error);
		}
	}

	async function handleResume() {
		timerStore.resume();
	}

	async function handleComplete() {
		try {
			await sessionCommands.complete(session.id);
			timerStore.stop();
		} catch (error) {
			console.error('Failed to complete session:', error);
		}
	}
</script>

<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
	<div class="mb-3 flex items-center justify-between">
		<div>
			<h3 class="text-lg font-semibold text-white capitalize">{session.session_type}</h3>
			<p class="text-sm text-gray-400">
				Planned: {session.planned_minutes} min
				{#if session.actual_minutes > 0}
					• Actual: {session.actual_minutes} min
				{/if}
			</p>
		</div>
		<div
			class="rounded-full px-3 py-1 text-xs font-medium"
			class:bg-blue-500/20={session.status === 'planned'}
			class:text-blue-400={session.status === 'planned'}
			class:bg-green-500/20={session.status === 'in_progress'}
			class:text-green-400={session.status === 'in_progress'}
			class:bg-gray-500/20={session.status === 'done'}
			class:text-gray-400={session.status === 'done'}
		>
			{session.status}
		</div>
	</div>

	{#if isActive}
		<div class="mb-4 text-center">
			<div class="text-4xl font-bold text-white">{formattedTime}</div>
			<div class="mt-1 text-sm text-gray-400">
				{elapsedMinutes} / {session.planned_minutes} minutes
			</div>
		</div>

		<div class="flex gap-2">
			{#if isRunning}
				<button
					onclick={handlePause}
					class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-yellow-600 px-4 py-2 text-white hover:bg-yellow-700"
				>
					<Icon icon="ph:pause-bold" width="20" />
					Pause
				</button>
			{:else}
				<button
					onclick={handleResume}
					class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-white hover:bg-green-700"
				>
					<Icon icon="ph:play-bold" width="20" />
					Resume
				</button>
			{/if}
			<button
				onclick={handleComplete}
				class="flex flex-1 items-center justify-center gap-2 rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"
			>
				<Icon icon="ph:check-bold" width="20" />
				Complete
			</button>
		</div>
	{:else if session.status === 'planned'}
		<button
			onclick={handleStart}
			class="flex w-full items-center justify-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-white hover:bg-green-700"
		>
			<Icon icon="ph:play-bold" width="20" />
			Start Session
		</button>
	{:else if session.status === 'done'}
		<div class="rounded-lg bg-green-500/10 p-3 text-center text-sm text-green-400">
			<Icon icon="ph:check-circle-bold" width="20" class="inline" />
			Completed in {session.actual_minutes} minutes
		</div>
	{/if}
</div>
