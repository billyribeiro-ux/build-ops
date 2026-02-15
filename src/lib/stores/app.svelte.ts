import type { Program, UserCapacityProfile } from '$lib/types';

interface Toast {
	id: string;
	message: string;
	type: 'success' | 'error' | 'warning' | 'info';
	duration: number;
}

class ToastStore {
	list = $state<Toast[]>([]);

	add(message: string, type: Toast['type'] = 'info', duration = 3000) {
		const id = crypto.randomUUID();
		this.list = [...this.list, { id, message, type, duration }];
	}

	remove(id: string) {
		this.list = this.list.filter((t) => t.id !== id);
	}

	success(message: string, duration = 3000) {
		this.add(message, 'success', duration);
	}

	error(message: string, duration = 5000) {
		this.add(message, 'error', duration);
	}

	warning(message: string, duration = 4000) {
		this.add(message, 'warning', duration);
	}

	info(message: string, duration = 3000) {
		this.add(message, 'info', duration);
	}
}

class AppStore {
	currentProgram = $state<Program | null>(null);
	capacityProfile = $state<UserCapacityProfile | null>(null);
	isLoading = $state(false);
	error = $state<string | null>(null);

	setProgram(program: Program | null) {
		this.currentProgram = program;
	}

	setCapacity(profile: UserCapacityProfile) {
		this.capacityProfile = profile;
	}

	setLoading(loading: boolean) {
		this.isLoading = loading;
	}

	setError(error: string | null) {
		this.error = error;
	}
}

export const appStore = new AppStore();
export const toasts = new ToastStore();
