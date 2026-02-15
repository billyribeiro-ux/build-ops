import { invoke } from '@tauri-apps/api/core';
import type { UserCapacityProfile, UpdateCapacityInput } from '$lib/types';

export async function getCapacityProfile(): Promise<UserCapacityProfile> {
	return invoke<UserCapacityProfile>('get_capacity_profile');
}

export async function updateCapacityProfile(
	input: UpdateCapacityInput
): Promise<UserCapacityProfile> {
	return invoke<UserCapacityProfile>('update_capacity_profile', { input });
}
