<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from './Button.svelte';

	interface Props {
		error?: Error | null;
		onRetry?: () => void;
		children?: import('svelte').Snippet;
	}

	let { error = null, onRetry, children }: Props = $props();
</script>

{#if error}
	<div class="flex min-h-[400px] items-center justify-center p-6">
		<div class="max-w-md text-center">
			<div class="mb-4 inline-flex rounded-full bg-red-500/10 p-4">
				<Icon icon="ph:warning-bold" width="48" class="text-red-500" />
			</div>
			<h2 class="mb-2 text-xl font-semibold text-white">Something went wrong</h2>
			<p class="mb-6 text-gray-400">{error.message}</p>
			{#if onRetry}
				<Button onclick={onRetry} icon="ph:arrow-clockwise-bold">
					Try Again
				</Button>
			{/if}
		</div>
	</div>
{:else if children}
	{@render children()}
{/if}
