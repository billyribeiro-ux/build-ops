export interface Program {
	id: string;
	title: string;
	description: string;
	target_days: number;
	status: 'active' | 'paused' | 'completed' | 'archived';
	created_at: string;
	updated_at: string;
}

export interface ProgramSummary extends Program {
	days_count: number;
	completed_days: number;
	latest_score: number | null;
}

export interface ProgramStats {
	total_days: number;
	completed_days: number;
	blocked_days: number;
	average_score: number;
	current_streak: number;
	total_time_minutes: number;
}

export interface CreateProgramInput {
	title: string;
	description: string;
	target_days: number;
}

export interface UpdateProgramInput {
	title?: string;
	description?: string;
	status?: string;
}
