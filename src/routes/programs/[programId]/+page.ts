import { getProgram } from '$lib/commands/programs';
import { listModules } from '$lib/commands/modules';

export const ssr = false;
export const prerender = false;

export async function load({ params }: { params: { programId: string } }) {
	try {
		const [program, modules] = await Promise.all([
			getProgram(params.programId),
			listModules(params.programId)
		]);
		
		return { program, modules };
	} catch (error) {
		console.error('Failed to load program:', error);
		throw error;
	}
}
