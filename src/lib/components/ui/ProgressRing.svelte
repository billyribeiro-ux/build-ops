<script lang="ts">
	interface Props {
		progress: number;
		size?: number;
		strokeWidth?: number;
		color?: string;
		showLabel?: boolean;
	}

	let {
		progress = 0,
		size = 120,
		strokeWidth = 8,
		color = '#3b82f6',
		showLabel = true
	}: Props = $props();

	let normalizedProgress = $derived(Math.min(100, Math.max(0, progress)));
	let radius = $derived((size - strokeWidth) / 2);
	let circumference = $derived(2 * Math.PI * radius);
	let offset = $derived(circumference - (normalizedProgress / 100) * circumference);
</script>

<div class="relative inline-flex items-center justify-center" style="width: {size}px; height: {size}px;">
	<svg class="transform -rotate-90" width={size} height={size}>
		<circle
			cx={size / 2}
			cy={size / 2}
			r={radius}
			stroke="currentColor"
			stroke-width={strokeWidth}
			fill="none"
			class="text-gray-700"
		/>
		<circle
			cx={size / 2}
			cy={size / 2}
			r={radius}
			stroke={color}
			stroke-width={strokeWidth}
			fill="none"
			stroke-dasharray={circumference}
			stroke-dashoffset={offset}
			stroke-linecap="round"
			class="transition-all duration-500 ease-out"
		/>
	</svg>
	{#if showLabel}
		<div class="absolute inset-0 flex items-center justify-center">
			<span class="text-2xl font-bold text-white">{Math.round(normalizedProgress)}%</span>
		</div>
	{/if}
</div>
