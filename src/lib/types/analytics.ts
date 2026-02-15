export interface TimeAnalytics {
	today_planned: number;
	today_actual: number;
	week_total: number;
	week_target: number;
	accuracy_trend: AccuracyPoint[];
	focus_efficiency: number;
	deep_days_used: number;
	deep_days_limit: number;
}

export interface AccuracyPoint {
	date: string;
	planned: number;
	actual: number;
	variance: number;
}

export interface FocusMetricsDaily {
	id: string;
	date: string;
	user_id: string;
	total_planned_minutes: number;
	total_actual_minutes: number;
	variance_percentage: number;
	completion_rate: number;
	focus_efficiency: number;
	deep_work_minutes: number;
	interruption_count: number;
	created_at: string;
	updated_at: string;
}
