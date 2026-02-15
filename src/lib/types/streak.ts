export interface Streak {
	id: string;
	program_id: string;
	current_streak: number;
	longest_streak: number;
	last_activity_date: string;
	freezes_available: number;
	freezes_used_this_month: number;
	created_at: string;
	updated_at: string;
}

export interface StreakFreeze {
	id: string;
	program_id: string;
	used_date: string;
	reason: string;
	created_at: string;
}
