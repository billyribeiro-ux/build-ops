import type { PageLoad } from './$types';
import { listPrograms } from '$lib/commands/programs';

export const ssr = false;
export const prerender = false;

export const load: PageLoad = async () => {
	try {
		const programs = await listPrograms();
		return { programs };
	} catch (error) {
		console.error('Failed to load programs:', error);
		return { programs: [], error: String(error) };
	}
};
