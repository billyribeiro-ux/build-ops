export interface DayPlan {
	id: string;
	program_id: string;
	module_id: string;
	title: string;
	day_number: number;
	version: number;
	status: 'draft' | 'published' | 'archived';
	syntax_targets: string;
	implementation_brief: string;
	files_to_create: string;
	success_criteria: string;
	stretch_challenge: string;
	notes: string;
	estimated_minutes: number;
	memory_rebuild_minutes: number;
	min_minutes: number;
	recommended_minutes: number;
	deep_minutes: number;
	complexity_level: number;
	focus_blocks: string;
	created_at: string;
	updated_at: string;
}

export interface FocusBlock {
	session_type: string;
	minutes: number;
}

export interface CreateDayPlanInput {
	program_id: string;
	module_id: string;
	title: string;
	day_number: number;
	syntax_targets: string;
	implementation_brief: string;
	min_minutes?: number;
	recommended_minutes?: number;
	deep_minutes?: number;
	complexity_level?: number;
}
