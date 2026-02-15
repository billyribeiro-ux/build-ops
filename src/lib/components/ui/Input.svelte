<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import Icon from '@iconify/svelte';

	let {
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		label = '',
		error = '',
		icon,
		disabled = false,
		id = '',
		class: className = ''
	}: {
		value?: string;
		type?: 'text' | 'email' | 'password' | 'number' | 'url' | 'search';
		placeholder?: string;
		label?: string;
		error?: string;
		icon?: string;
		disabled?: boolean;
		id?: string;
		class?: string;
	} = $props();
</script>

<div class={cn('space-y-1.5', className)}>
	{#if label}
		<label for={id} class="block text-sm font-medium text-gray-400">{label}</label>
	{/if}
	<div class="relative">
		{#if icon}
			<div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
				<Icon {icon} width="16" class="text-gray-500" />
			</div>
		{/if}
		<input
			{id}
			{type}
			{placeholder}
			{disabled}
			bind:value
			class={cn(
				'w-full rounded-lg border bg-gray-800 px-4 py-2.5 text-sm text-white placeholder-gray-500 transition focus:outline-none focus:ring-2 focus:ring-offset-1 focus:ring-offset-gray-950',
				icon ? 'pl-10' : '',
				error ? 'border-red-500 focus:ring-red-500' : 'border-gray-700 focus:border-indigo-500 focus:ring-indigo-500',
				disabled ? 'opacity-50 cursor-not-allowed' : ''
			)}
		/>
	</div>
	{#if error}
		<p class="text-xs text-red-400">{error}</p>
	{/if}
</div>
