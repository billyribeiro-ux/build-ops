export interface DayAttempt {
	id: string;
	day_plan_id: string;
	attempt_number: number;
	status: 'in_progress' | 'passed' | 'blocked' | 'mastery';
	is_draft: boolean;
	score_implementation: number;
	score_code_quality: number;
	score_accessibility: number;
	score_performance: number;
	score_quiz: number;
	total_score: number;
	daily_summary: string;
	what_went_well: string;
	what_to_improve: string;
	key_learnings: string;
	memory_rebuild_passed: boolean;
	memory_rebuild_notes: string;
	actual_minutes: number;
	started_at: string;
	submitted_at: string | null;
	last_autosave: string;
	created_at: string;
	updated_at: string;
}

export interface CreateDayAttemptInput {
	day_plan_id: string;
}

export interface UpdateDayAttemptInput {
	daily_summary?: string;
	what_went_well?: string;
	what_to_improve?: string;
	key_learnings?: string;
	memory_rebuild_passed?: boolean;
	memory_rebuild_notes?: string;
}

export interface SubmitScoresInput {
	score_implementation: number;
	score_code_quality: number;
	score_accessibility: number;
	score_performance: number;
	score_quiz: number;
	daily_summary: string;
	what_went_well: string;
	what_to_improve: string;
	key_learnings: string;
	memory_rebuild_passed: boolean;
	memory_rebuild_notes: string;
}

export interface DayAttemptSummary {
	id: string;
	day_plan_id: string;
	day_title: string;
	day_number: number;
	attempt_number: number;
	status: string;
	total_score: number;
	memory_rebuild_passed: boolean;
	actual_minutes: number;
	submitted_at: string | null;
	created_at: string;
}
