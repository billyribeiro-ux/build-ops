import { listDayPlansByModule } from '$lib/commands';

export const ssr = false;
export const prerender = false;

export async function load({ params }: { params: { programId: string } }) {
	try {
		const moduleId = params.programId;
		const dayPlans = await listDayPlansByModule(moduleId);
		return { dayPlans, moduleId };
	} catch (error) {
		console.error('Failed to load day plans:', error);
		return { dayPlans: [], moduleId: params.programId, error: String(error) };
	}
}
