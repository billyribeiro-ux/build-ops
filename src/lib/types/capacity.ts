export interface UserCapacityProfile {
	id: string;
	user_id: string;
	default_daily_minutes: number;
	weekly_study_days: number;
	preferred_start_time: string;
	max_deep_days_per_week: number;
	break_pattern: string;
	timezone: string;
	created_at: string;
	updated_at: string;
}

export interface UpdateCapacityInput {
	default_daily_minutes?: number;
	weekly_study_days?: number;
	preferred_start_time?: string;
	max_deep_days_per_week?: number;
	break_pattern?: string;
	timezone?: string;
}
