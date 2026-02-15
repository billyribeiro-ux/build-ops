export interface DaySession {
	id: string;
	day_attempt_id: string;
	session_type: 'learn' | 'build' | 'debug' | 'rebuild' | 'quiz' | 'review';
	planned_minutes: number;
	actual_minutes: number;
	started_at: string | null;
	ended_at: string | null;
	status: 'planned' | 'in_progress' | 'done' | 'skipped';
	notes: string;
	created_at: string;
	updated_at: string;
}

export interface CreateSessionInput {
	day_attempt_id: string;
	session_type: string;
	planned_minutes: number;
}

export interface GeneratedPlan {
	sessions: PlannedSession[];
	total_minutes: number;
	estimated_end_time: string;
}

export interface PlannedSession {
	session_type: string;
	planned_minutes: number;
	start_time: string;
	end_time: string;
	description: string;
}

export interface SessionInterruption {
	id: string;
	session_id: string;
	interruption_type: 'external' | 'mental' | 'technical' | 'break';
	duration_seconds: number;
	notes: string;
	occurred_at: string;
}
