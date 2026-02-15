<script lang="ts">
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Icon from '@iconify/svelte';

	let exportType = $state<'pdf' | 'json' | 'csv'>('pdf');
	let isExporting = $state(false);

	async function handleExport() {
		isExporting = true;
		try {
			await new Promise(resolve => setTimeout(resolve, 1500));
			alert(`${exportType.toUpperCase()} export functionality will be implemented`);
		} finally {
			isExporting = false;
		}
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-4xl">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-white">Export Center</h1>
			<p class="mt-1 text-gray-400">Export your data in various formats</p>
		</div>

		<div class="grid gap-6 md:grid-cols-3">
			<Card>
				<button
					onclick={() => exportType = 'pdf'}
					class="w-full p-6 text-left transition-colors {exportType === 'pdf' ? 'bg-blue-500/10' : 'hover:bg-gray-800/50'}"
				>
					<div class="mb-4 inline-flex rounded-lg bg-red-500/10 p-3">
						<Icon icon="ph:file-pdf-bold" width="32" class="text-red-500" />
					</div>
					<h3 class="mb-2 text-lg font-semibold text-white">PDF Report</h3>
					<p class="text-sm text-gray-400">
						Generate comprehensive program reports with charts, scores, and analytics
					</p>
					{#if exportType === 'pdf'}
						<div class="mt-4 flex items-center gap-2 text-sm text-blue-400">
							<Icon icon="ph:check-circle-bold" width="16" />
							<span>Selected</span>
						</div>
					{/if}
				</button>
			</Card>

			<Card>
				<button
					onclick={() => exportType = 'json'}
					class="w-full p-6 text-left transition-colors {exportType === 'json' ? 'bg-blue-500/10' : 'hover:bg-gray-800/50'}"
				>
					<div class="mb-4 inline-flex rounded-lg bg-blue-500/10 p-3">
						<Icon icon="ph:file-code-bold" width="32" class="text-blue-500" />
					</div>
					<h3 class="mb-2 text-lg font-semibold text-white">JSON Backup</h3>
					<p class="text-sm text-gray-400">
						Export complete program data for backup or migration purposes
					</p>
					{#if exportType === 'json'}
						<div class="mt-4 flex items-center gap-2 text-sm text-blue-400">
							<Icon icon="ph:check-circle-bold" width="16" />
							<span>Selected</span>
						</div>
					{/if}
				</button>
			</Card>

			<Card>
				<button
					onclick={() => exportType = 'csv'}
					class="w-full p-6 text-left transition-colors {exportType === 'csv' ? 'bg-blue-500/10' : 'hover:bg-gray-800/50'}"
				>
					<div class="mb-4 inline-flex rounded-lg bg-green-500/10 p-3">
						<Icon icon="ph:file-csv-bold" width="32" class="text-green-500" />
					</div>
					<h3 class="mb-2 text-lg font-semibold text-white">CSV Data</h3>
					<p class="text-sm text-gray-400">
						Export tabular data for analysis in spreadsheet applications
					</p>
					{#if exportType === 'csv'}
						<div class="mt-4 flex items-center gap-2 text-sm text-blue-400">
							<Icon icon="ph:check-circle-bold" width="16" />
							<span>Selected</span>
						</div>
					{/if}
				</button>
			</Card>
		</div>

		<Card class="mt-6">
			<div class="p-6">
				<h2 class="mb-4 text-xl font-semibold text-white">Export Options</h2>
				
				{#if exportType === 'pdf'}
					<div class="space-y-4">
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-charts" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-charts" class="text-sm text-gray-300">Include analytics charts</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-scores" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-scores" class="text-sm text-gray-300">Include score details</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-bugs" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-bugs" class="text-sm text-gray-300">Include bug logs</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-evidence" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" />
							<label for="include-evidence" class="text-sm text-gray-300">Include evidence artifacts</label>
						</div>
					</div>
				{:else if exportType === 'json'}
					<div class="space-y-4">
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-programs" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-programs" class="text-sm text-gray-300">Include all programs</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-attempts" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-attempts" class="text-sm text-gray-300">Include attempt history</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-artifacts" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-artifacts" class="text-sm text-gray-300">Include artifacts</label>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="pretty-print" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" />
							<label for="pretty-print" class="text-sm text-gray-300">Pretty print JSON</label>
						</div>
					</div>
				{:else}
					<div class="space-y-4">
						<div>
							<label class="block text-sm font-medium text-gray-300">Export Data</label>
							<select class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white">
								<option>Attempt scores</option>
								<option>Time tracking</option>
								<option>Bug logs</option>
								<option>Exercise entries</option>
							</select>
						</div>
						<div class="flex items-center gap-3">
							<input type="checkbox" id="include-headers" class="h-4 w-4 rounded border-gray-600 bg-gray-700 text-blue-500" checked />
							<label for="include-headers" class="text-sm text-gray-300">Include column headers</label>
						</div>
					</div>
				{/if}

				<div class="mt-6 flex gap-3">
					<Button
						onclick={handleExport}
						disabled={isExporting}
						icon={isExporting ? 'ph:spinner-bold' : 'ph:download-bold'}
						class="flex-1"
					>
						{isExporting ? 'Exporting...' : `Export as ${exportType.toUpperCase()}`}
					</Button>
					<Button variant="outline" icon="ph:x-bold">
						Cancel
					</Button>
				</div>
			</div>
		</Card>

		<Card class="mt-6">
			<div class="p-6">
				<h2 class="mb-4 text-xl font-semibold text-white">Recent Exports</h2>
				<div class="space-y-3">
					<div class="flex items-center justify-between rounded-lg border border-gray-700 bg-gray-800 p-4">
						<div class="flex items-center gap-3">
							<Icon icon="ph:file-pdf-bold" width="24" class="text-red-500" />
							<div>
								<p class="font-medium text-white">Program Report - React Mastery</p>
								<p class="text-sm text-gray-400">Exported 2 days ago • 2.4 MB</p>
							</div>
						</div>
						<Button variant="outline" size="sm" icon="ph:download-bold">
							Download
						</Button>
					</div>
					<div class="flex items-center justify-between rounded-lg border border-gray-700 bg-gray-800 p-4">
						<div class="flex items-center gap-3">
							<Icon icon="ph:file-code-bold" width="24" class="text-blue-500" />
							<div>
								<p class="font-medium text-white">Full Backup - All Programs</p>
								<p class="text-sm text-gray-400">Exported 1 week ago • 8.7 MB</p>
							</div>
						</div>
						<Button variant="outline" size="sm" icon="ph:download-bold">
							Download
						</Button>
					</div>
				</div>
			</div>
		</Card>
	</div>
</div>
