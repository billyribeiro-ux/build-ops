<script lang="ts">
	import './layout.css';
	import Icon from '@iconify/svelte';
	import { page } from '$app/stores';

	let { children } = $props();

	let currentPath = $derived($page.url.pathname);

	const navItems = [
		{ href: '/', icon: 'ph:house-bold', label: 'Dashboard' },
		{ href: '/import', icon: 'ph:upload-bold', label: 'Import' },
	];
</script>

<div class="flex h-screen overflow-hidden bg-gray-950">
	<aside class="flex w-56 flex-col border-r border-gray-800 bg-gray-900">
		<div class="flex h-14 items-center gap-2 border-b border-gray-800 px-5">
			<Icon icon="ph:cube-bold" width="22" class="text-indigo-400" />
			<span class="text-lg font-bold text-white">BuildOps</span>
			<span class="rounded bg-indigo-500/20 px-1.5 py-0.5 text-[10px] font-semibold text-indigo-300">4.0</span>
		</div>

		<nav class="flex-1 space-y-1 px-3 py-4">
			{#each navItems as item (item.href)}
				<a
					href={item.href}
					class="flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors
						{currentPath === item.href
							? 'bg-indigo-500/15 text-indigo-300'
							: 'text-gray-400 hover:bg-gray-800 hover:text-gray-200'}"
				>
					<Icon icon={item.icon} width="18" />
					{item.label}
				</a>
			{/each}
		</nav>

		<div class="border-t border-gray-800 px-4 py-3">
			<div class="text-xs text-gray-500">v0.1.0 • Desktop</div>
		</div>
	</aside>

	<main class="flex-1 overflow-y-auto">
		{@render children()}
	</main>
</div>
