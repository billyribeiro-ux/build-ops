<script lang="ts">
	import Icon from '@iconify/svelte';

	interface Props {
		size?: number;
		message?: string;
	}

	let { size = 48, message = '' }: Props = $props();
</script>

<div class="flex flex-col items-center justify-center gap-3">
	<Icon icon="ph:spinner-bold" width={size} class="animate-spin text-blue-500" />
	{#if message}
		<p class="text-sm text-gray-400">{message}</p>
	{/if}
</div>
