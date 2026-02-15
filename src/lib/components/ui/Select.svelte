<script lang="ts">
	import { cn } from '$lib/utils/cn';

	let {
		value = $bindable(''),
		options = [],
		label = '',
		error = '',
		disabled = false,
		id = '',
		placeholder = 'Select...',
		class: className = ''
	}: {
		value?: string;
		options?: { value: string; label: string }[];
		label?: string;
		error?: string;
		disabled?: boolean;
		id?: string;
		placeholder?: string;
		class?: string;
	} = $props();
</script>

<div class={cn('space-y-1.5', className)}>
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-400">{label}</label>
	{/if}
	<select
		{id}
		{disabled}
		bind:value
		class={cn(
			'w-full rounded-lg border bg-gray-800 px-4 py-2.5 text-sm text-white transition focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-offset-gray-950 appearance-none',
			error ? 'border-red-500 focus:ring-red-500' : 'border-gray-700 focus:border-indigo-500 focus:ring-indigo-500',
			disabled ? 'opacity-50 cursor-not-allowed' : ''
		)}
	>
		<option value="" disabled>{placeholder}</option>
		{#each options as opt (opt.value)}
			<option value={opt.value}>{opt.label}</option>
		{/each}
	</select>
	{#if error}
		<p class="text-xs text-red-400">{error}</p>
	{/if}
</div>
