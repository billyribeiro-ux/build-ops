import type { Program, UserCapacityProfile } from '$lib/types';

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
