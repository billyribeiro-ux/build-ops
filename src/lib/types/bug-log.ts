export interface BugLog {
	id: string;
	day_attempt_id: string;
	category: string;
	severity: 'low' | 'medium' | 'high' | 'critical';
	symptom: string;
	root_cause: string;
	fix_applied: string;
	prevention_strategy: string;
	time_to_fix_minutes: number;
	created_at: string;
	updated_at: string;
}

export interface CreateBugLogInput {
	day_attempt_id: string;
	category: string;
	severity: string;
	symptom: string;
	root_cause: string;
	fix_applied: string;
	prevention_strategy: string;
	time_to_fix_minutes: number;
}

export interface UpdateBugLogInput {
	category?: string;
	severity?: string;
	symptom?: string;
	root_cause?: string;
	fix_applied?: string;
	prevention_strategy?: string;
	time_to_fix_minutes?: number;
}
