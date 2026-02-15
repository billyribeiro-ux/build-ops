<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import Icon from '@iconify/svelte';
	import type { Snippet } from 'svelte';

	let {
		variant = 'primary',
		size = 'md',
		icon,
		iconRight,
		disabled = false,
		loading = false,
		class: className = '',
		onclick,
		type = 'button',
		children
	}: {
		variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' | 'success';
		size?: 'sm' | 'md' | 'lg' | 'small' | 'medium' | 'large';
		icon?: string;
		iconRight?: string;
		disabled?: boolean;
		loading?: boolean;
		class?: string;
		onclick?: (e: MouseEvent) => void;
		type?: 'button' | 'submit' | 'reset';
		children?: Snippet;
	} = $props();

	const normalizedSize = $derived(size === 'small' ? 'sm' : size === 'medium' ? 'md' : size === 'large' ? 'lg' : size);

	const variantClasses: Record<string, string> = {
		primary: 'bg-indigo-600 text-white hover:bg-indigo-700 focus:ring-indigo-500',
		secondary: 'bg-gray-800 text-gray-200 border border-gray-700 hover:bg-gray-700 focus:ring-gray-500',
		outline: 'border border-border-primary text-text-primary hover:bg-bg-hover focus:ring-accent-primary',
		ghost: 'text-gray-400 hover:bg-gray-800 hover:text-gray-200 focus:ring-gray-500',
		danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500',
		success: 'bg-green-600 text-white hover:bg-green-700 focus:ring-green-500'
	};

	const sizeClasses: Record<string, string> = {
		sm: 'px-3 py-1.5 text-xs gap-1.5',
		md: 'px-4 py-2.5 text-sm gap-2',
		lg: 'px-6 py-3 text-base gap-2.5'
	};

	const iconSizes: Record<string, number> = { sm: 14, md: 16, lg: 18 };
</script>

<button
	{type}
	{onclick}
	disabled={disabled || loading}
	class={cn(
		'inline-flex items-center justify-center rounded-lg font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-950 disabled:opacity-50 disabled:cursor-not-allowed',
		variantClasses[variant],
		sizeClasses[normalizedSize],
		className
	)}
>
	{#if loading}
		<Icon icon="ph:spinner-bold" width={iconSizes[normalizedSize]} class="animate-spin" />
	{:else if icon}
		<Icon {icon} width={iconSizes[normalizedSize]} />
	{/if}
	{#if children}
		{@render children()}
	{/if}
	{#if iconRight && !loading}
		<Icon icon={iconRight} width={iconSizes[normalizedSize]} />
	{/if}
</button>
