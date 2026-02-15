<script lang="ts">
	import Card from '$lib/components/ui/Card.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';

	let selectedTimeRange = $state('7d');
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-7xl">
		<div class="mb-8 flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-white">Analytics</h1>
				<p class="mt-1 text-gray-400">Comprehensive insights into your learning progress</p>
			</div>
			<div class="flex gap-2">
				<button
					onclick={() => selectedTimeRange = '7d'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === '7d' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					7 Days
				</button>
				<button
					onclick={() => selectedTimeRange = '30d'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === '30d' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					30 Days
				</button>
				<button
					onclick={() => selectedTimeRange = 'all'}
					class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {selectedTimeRange === 'all' ? 'bg-blue-500 text-white' : 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
				>
					All Time
				</button>
			</div>
		</div>

		<div class="grid grid-cols-2 gap-6">
			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Skill Radar</h2>
						<Badge variant="info">6 domains</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:chart-polar-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Radar chart visualization</p>
						</div>
					</div>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Concept Heatmap</h2>
						<Badge variant="warning">Activity map</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:squares-four-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Heatmap visualization</p>
						</div>
					</div>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Score Trends</h2>
						<Badge variant="success">Improving</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:chart-line-up-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Line chart visualization</p>
						</div>
					</div>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Burndown Chart</h2>
						<Badge variant="purple">On track</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:chart-line-down-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Burndown visualization</p>
						</div>
					</div>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Time Distribution</h2>
						<Badge variant="info">Hours tracked</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:clock-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Time chart visualization</p>
						</div>
					</div>
				</div>
			</Card>

			<Card>
				<div class="p-6">
					<div class="mb-4 flex items-center justify-between">
						<h2 class="text-lg font-semibold text-white">Bug Patterns</h2>
						<Badge variant="danger">Analysis</Badge>
					</div>
					<div class="flex h-64 items-center justify-center rounded-lg bg-gray-800/50">
						<div class="text-center">
							<Icon icon="ph:bug-bold" width="48" class="mx-auto text-gray-600" />
							<p class="mt-2 text-sm text-gray-500">Bug pattern visualization</p>
						</div>
					</div>
				</div>
			</Card>
		</div>
	</div>
</div>
