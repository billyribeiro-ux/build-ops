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
		min,
		max,
		class: className = '',
		...rest
	}: {
		type?: string;
		value?: string | number;
		placeholder?: string;
		label?: string;
		error?: string;
		icon?: string;
		disabled?: boolean;
		id?: string;
		min?: number;
		max?: number;
		class?: string;
		[key: string]: any;
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
			bind:value
			{placeholder}
			{disabled}
			{min}
			{max}
			{...rest}
			class={cn(
				'w-full rounded-lg border bg-bg-secondary px-4 py-2.5 text-sm text-text-primary placeholder-text-tertiary transition-colors focus:outline-none focus:ring-2 focus:ring-accent-primary disabled:cursor-not-allowed disabled:opacity-50',
				error ? 'border-accent-danger focus:ring-accent-danger' : 'border-border-primary',
				className
			)}
		/>
	</div>
	{#if error}
		<p class="text-xs text-red-400">{error}</p>
	{/if}
</div>
