<script lang="ts">
	import SessionTimer from './SessionTimer.svelte';
	import Icon from '@iconify/svelte';
	import type { DaySession } from '$lib/types';

	let { sessions }: { sessions: DaySession[] } = $props();

	let totalPlanned = $derived(sessions.reduce((sum, s) => sum + s.planned_minutes, 0));
	let totalActual = $derived(sessions.reduce((sum, s) => sum + s.actual_minutes, 0));
	let completedCount = $derived(sessions.filter((s) => s.status === 'done').length);
</script>

<div class="space-y-4">
	<div class="rounded-lg border border-gray-700 bg-gray-800 p-4">
		<div class="flex items-center justify-between">
			<div>
				<h3 class="text-lg font-semibold text-white">Session Progress</h3>
				<p class="text-sm text-gray-400">
					{completedCount} of {sessions.length} sessions completed
				</p>
			</div>
			<div class="text-right">
				<div class="text-2xl font-bold text-white">{totalActual} min</div>
				<div class="text-sm text-gray-400">of {totalPlanned} min planned</div>
			</div>
		</div>
		<div class="mt-3 h-2 overflow-hidden rounded-full bg-gray-700">
			<div
				class="h-full bg-gradient-to-r from-blue-500 to-purple-500 transition-all"
				style="width: {Math.min((totalActual / totalPlanned) * 100, 100)}%"
			></div>
		</div>
	</div>

	<div class="space-y-3">
		{#each sessions as session (session.id)}
			<SessionTimer {session} />
		{/each}
	</div>

	{#if sessions.length === 0}
		<div class="rounded-lg border border-dashed border-gray-700 bg-gray-800/50 p-8 text-center">
			<Icon icon="ph:calendar-blank-bold" width="48" class="mx-auto text-gray-600" />
			<p class="mt-3 text-gray-400">No sessions planned yet</p>
			<p class="mt-1 text-sm text-gray-500">Click "Plan My Day" to generate a schedule</p>
		</div>
	{/if}
</div>
