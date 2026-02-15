<script lang="ts">
	import { cn } from '$lib/utils/cn';

	let {
		checked = $bindable(false),
		label = '',
		disabled = false,
		id = '',
		class: className = ''
	}: {
		checked?: boolean;
		label?: string;
		disabled?: boolean;
		id?: string;
		class?: string;
	} = $props();
</script>

<label class={cn('inline-flex items-center gap-2.5 cursor-pointer', disabled ? 'opacity-50 cursor-not-allowed' : '', className)}>
	<input
		{id}
		type="checkbox"
		bind:checked
		{disabled}
		class="h-4 w-4 rounded border-gray-600 bg-gray-800 text-indigo-600 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-gray-950"
	/>
	{#if label}
		<span class="text-sm text-gray-300">{label}</span>
	{/if}
</label>
