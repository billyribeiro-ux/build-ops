export interface DayAttempt {
	id: string;
	day_plan_id: string;
	day_plan_version: number;
	attempt_number: number;
	status: 'in_progress' | 'submitted' | 'blocked' | 'passed' | 'mastery';
	score_implementation: number;
	score_code_quality: number;
	score_accessibility: number;
	score_performance: number;
	score_quiz: number;
	total_score: number;
	memory_rebuild_completed: number;
	memory_rebuild_passed: number;
	memory_rebuild_notes: string;
	what_broke: string;
	why_broke: string;
	how_fixed: string;
	refactor_tomorrow: string;
	daily_summary: string;
	exercise_notes: string;
	code_snapshot: string;
	started_at: string;
	submitted_at: string | null;
	actual_minutes: number;
	is_draft: number;
	last_autosave: string;
	created_at: string;
	updated_at: string;
}

export interface UpdateAttemptInput {
	status?: string;
	score_implementation?: number;
	score_code_quality?: number;
	score_accessibility?: number;
	score_performance?: number;
	score_quiz?: number;
	memory_rebuild_completed?: number;
	memory_rebuild_passed?: number;
	memory_rebuild_notes?: string;
	what_broke?: string;
	why_broke?: string;
	how_fixed?: string;
	refactor_tomorrow?: string;
	daily_summary?: string;
	exercise_notes?: string;
	code_snapshot?: string;
	actual_minutes?: number;
}
