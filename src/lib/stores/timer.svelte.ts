import type { DaySession } from '$lib/types';

class TimerStore {
	activeSession = $state<DaySession | null>(null);
	elapsedSeconds = $state(0);
	isRunning = $state(false);
	intervalId: number | null = null;

	start(session: DaySession) {
		this.activeSession = session;
		this.isRunning = true;
		this.elapsedSeconds = 0;

		if (this.intervalId) {
			clearInterval(this.intervalId);
		}

		this.intervalId = window.setInterval(() => {
			this.elapsedSeconds++;
		}, 1000);
	}

	pause() {
		this.isRunning = false;
		if (this.intervalId) {
			clearInterval(this.intervalId);
			this.intervalId = null;
		}
	}

	resume() {
		this.isRunning = true;
		this.intervalId = window.setInterval(() => {
			this.elapsedSeconds++;
		}, 1000);
	}

	stop() {
		this.isRunning = false;
		if (this.intervalId) {
			clearInterval(this.intervalId);
			this.intervalId = null;
		}
		this.activeSession = null;
		this.elapsedSeconds = 0;
	}

	get formattedTime() {
		const hours = Math.floor(this.elapsedSeconds / 3600);
		const minutes = Math.floor((this.elapsedSeconds % 3600) / 60);
		const seconds = this.elapsedSeconds % 60;

		if (hours > 0) {
			return `${hours}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
		}
		return `${minutes}:${seconds.toString().padStart(2, '0')}`;
	}

	get elapsedMinutes() {
		return Math.floor(this.elapsedSeconds / 60);
	}
}

export const timerStore = new TimerStore();
