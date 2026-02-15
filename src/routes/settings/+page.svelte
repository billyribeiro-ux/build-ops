<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Card from '$lib/components/ui/Card.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Badge from '$lib/components/ui/Badge.svelte';
	import Icon from '@iconify/svelte';
	import { getCapacityProfile, updateCapacityProfile } from '$lib/commands';
	import type { UserCapacityProfile } from '$lib/types';

	let activeSection = $state<'general' | 'capacity' | 'notifications' | 'data'>('general');
	let theme = $state('dark');
	let defaultDailyMinutes = $state(60);
	let weeklyStudyDays = $state(5);
	let preferredStartTime = $state('09:00');
	let maxDeepDays = $state(3);
	let breakPattern = $state('25/5');
	let enableNotifications = $state(true);
	let enableStreakReminders = $state(true);
	let autoBackup = $state(true);
	let isSaving = $state(false);
	let saveMessage = $state('');

	onMount(async () => {
		try {
			const profile = await getCapacityProfile();
			defaultDailyMinutes = profile.default_daily_minutes;
			weeklyStudyDays = profile.weekly_study_days;
			preferredStartTime = profile.preferred_start_time;
			maxDeepDays = profile.max_deep_days_per_week;
			breakPattern = profile.break_pattern;
		} catch {
			console.log('No capacity profile found, using defaults');
		}
	});

	async function saveCapacity() {
		isSaving = true;
		saveMessage = '';
		try {
			await updateCapacityProfile({
				default_daily_minutes: defaultDailyMinutes,
				weekly_study_days: weeklyStudyDays,
				preferred_start_time: preferredStartTime,
				max_deep_days_per_week: maxDeepDays,
				break_pattern: breakPattern
			});
			saveMessage = 'Settings saved!';
			setTimeout(() => saveMessage = '', 2000);
		} catch (err) {
			saveMessage = 'Failed to save';
			console.error('Save failed:', err);
		} finally {
			isSaving = false;
		}
	}

	function handleExportData() {
		goto('/export');
	}

	function handleImportData() {
		goto('/import');
	}

	function handleClearData() {
		if (confirm('Are you sure you want to clear all data? This cannot be undone.')) {
			alert('Clear data requires manual database deletion');
		}
	}
</script>

<div class="min-h-screen bg-gray-950 p-6">
	<div class="mx-auto max-w-6xl">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-white">Settings</h1>
			<p class="mt-1 text-gray-400">Manage your BuildOps 40 preferences</p>
		</div>

		<div class="grid grid-cols-4 gap-6">
			<div class="col-span-1">
				<Card>
					<div class="p-4">
						<nav class="space-y-1">
							<button
								onclick={() => activeSection = 'general'}
								class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors {activeSection === 'general' ? 'bg-blue-500/10 text-blue-400' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
							>
								<Icon icon="ph:gear-bold" width="20" />
								<span class="text-sm font-medium">General</span>
							</button>
							<button
								onclick={() => activeSection = 'capacity'}
								class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors {activeSection === 'capacity' ? 'bg-blue-500/10 text-blue-400' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
							>
								<Icon icon="ph:clock-bold" width="20" />
								<span class="text-sm font-medium">Capacity</span>
							</button>
							<button
								onclick={() => activeSection = 'notifications'}
								class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors {activeSection === 'notifications' ? 'bg-blue-500/10 text-blue-400' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
							>
								<Icon icon="ph:bell-bold" width="20" />
								<span class="text-sm font-medium">Notifications</span>
							</button>
							<button
								onclick={() => activeSection = 'data'}
								class="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors {activeSection === 'data' ? 'bg-blue-500/10 text-blue-400' : 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
							>
								<Icon icon="ph:database-bold" width="20" />
								<span class="text-sm font-medium">Data</span>
							</button>
						</nav>
					</div>
				</Card>
			</div>

			<div class="col-span-3">
				{#if activeSection === 'general'}
					<Card>
						<div class="p-6">
							<h2 class="mb-6 text-xl font-semibold text-white">General Settings</h2>
							
							<div class="space-y-6">
								<div>
									<label class="block text-sm font-medium text-gray-300">Theme</label>
									<select
										bind:value={theme}
										class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									>
										<option value="dark">Dark</option>
										<option value="light">Light</option>
										<option value="system">System</option>
									</select>
									<p class="mt-1 text-xs text-gray-500">Choose your preferred color scheme</p>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">Language</label>
									<select
										class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									>
										<option value="en">English</option>
									</select>
									<p class="mt-1 text-xs text-gray-500">Application language</p>
								</div>

								<div class="border-t border-gray-700 pt-6">
									<h3 class="mb-4 text-sm font-semibold text-white">About</h3>
									<div class="space-y-2 text-sm">
										<div class="flex justify-between">
											<span class="text-gray-400">Version</span>
											<span class="text-white">0.1.0</span>
										</div>
										<div class="flex justify-between">
											<span class="text-gray-400">Build</span>
											<span class="text-white">Development</span>
										</div>
									</div>
								</div>
							</div>
						</div>
					</Card>
				{:else if activeSection === 'capacity'}
					<Card>
						<div class="p-6">
							<h2 class="mb-6 text-xl font-semibold text-white">Capacity Settings</h2>
							
							<div class="space-y-6">
								<div>
									<label class="block text-sm font-medium text-gray-300">Default Daily Minutes</label>
									<input type="number" bind:value={defaultDailyMinutes} min="15" max="480" class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500" />
									<p class="mt-1 text-xs text-gray-500">Your typical daily learning capacity in minutes</p>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">Weekly Study Days</label>
									<input type="number" bind:value={weeklyStudyDays} min="1" max="7" class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500" />
									<p class="mt-1 text-xs text-gray-500">How many days per week you study</p>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">Preferred Start Time</label>
									<input type="time" bind:value={preferredStartTime} class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500" />
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">Max Deep Work Days/Week</label>
									<input type="number" bind:value={maxDeepDays} min="0" max="7" class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500" />
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">Break Pattern</label>
									<select bind:value={breakPattern} class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500">
										<option value="25/5">Pomodoro (25/5)</option>
										<option value="50/10">Long Focus (50/10)</option>
										<option value="90/20">Deep Work (90/20)</option>
									</select>
								</div>

								<div class="flex items-center gap-3 border-t border-gray-700 pt-6">
									<Button onclick={saveCapacity} icon="ph:floppy-disk-bold" disabled={isSaving}>
										{isSaving ? 'Saving...' : 'Save Settings'}
									</Button>
									{#if saveMessage}
										<span class="text-sm {saveMessage === 'Settings saved!' ? 'text-green-400' : 'text-red-400'}">{saveMessage}</span>
									{/if}
								</div>
							</div>
						</div>
					</Card>
				{:else if activeSection === 'notifications'}
					<Card>
						<div class="p-6">
							<h2 class="mb-6 text-xl font-semibold text-white">Notification Settings</h2>
							
							<div class="space-y-6">
								<div class="flex items-center justify-between">
									<div>
										<p class="font-medium text-white">Enable Notifications</p>
										<p class="text-sm text-gray-400">Receive app notifications</p>
									</div>
									<button
										onclick={() => enableNotifications = !enableNotifications}
										class="relative h-6 w-11 rounded-full transition-colors {enableNotifications ? 'bg-blue-500' : 'bg-gray-700'}"
									>
										<span class="absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-white transition-transform {enableNotifications ? 'translate-x-5' : ''}"></span>
									</button>
								</div>

								<div class="flex items-center justify-between">
									<div>
										<p class="font-medium text-white">Streak Reminders</p>
										<p class="text-sm text-gray-400">Daily reminders to maintain your streak</p>
									</div>
									<button
										onclick={() => enableStreakReminders = !enableStreakReminders}
										class="relative h-6 w-11 rounded-full transition-colors {enableStreakReminders ? 'bg-blue-500' : 'bg-gray-700'}"
									>
										<span class="absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-white transition-transform {enableStreakReminders ? 'translate-x-5' : ''}"></span>
									</button>
								</div>

								<div>
									<label class="block text-sm font-medium text-gray-300">
										Reminder Time
									</label>
									<input
										type="time"
										value="09:00"
										class="mt-2 w-full rounded-lg border border-gray-700 bg-gray-800 px-4 py-2 text-white focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
									/>
									<p class="mt-1 text-xs text-gray-500">When to send daily reminders</p>
								</div>
							</div>
						</div>
					</Card>
				{:else if activeSection === 'data'}
					<Card>
						<div class="p-6">
							<h2 class="mb-6 text-xl font-semibold text-white">Data Management</h2>
							
							<div class="space-y-6">
								<div class="flex items-center justify-between">
									<div>
										<p class="font-medium text-white">Auto Backup</p>
										<p class="text-sm text-gray-400">Automatically backup your data daily</p>
									</div>
									<button
										onclick={() => autoBackup = !autoBackup}
										class="relative h-6 w-11 rounded-full transition-colors {autoBackup ? 'bg-blue-500' : 'bg-gray-700'}"
									>
										<span class="absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-white transition-transform {autoBackup ? 'translate-x-5' : ''}"></span>
									</button>
								</div>

								<div class="border-t border-gray-700 pt-6">
									<h3 class="mb-4 text-sm font-semibold text-white">Export & Import</h3>
									<div class="space-y-3">
										<Button onclick={handleExportData} icon="ph:download-bold" class="w-full">
											Export All Data
										</Button>
										<Button onclick={handleImportData} variant="outline" icon="ph:upload-bold" class="w-full">
											Import Data
										</Button>
									</div>
								</div>

								<div class="border-t border-gray-700 pt-6">
									<h3 class="mb-4 text-sm font-semibold text-white">Danger Zone</h3>
									<Button onclick={handleClearData} variant="outline" icon="ph:trash-bold" class="w-full border-red-500 text-red-500 hover:bg-red-500/10">
										Clear All Data
									</Button>
									<p class="mt-2 text-xs text-gray-500">This action cannot be undone</p>
								</div>
							</div>
						</div>
					</Card>
				{/if}
			</div>
		</div>
	</div>
</div>
