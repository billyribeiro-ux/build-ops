export interface TimeRecommendation {
	id: string;
	user_id: string;
	recommendation_type: 'increase_build' | 'decrease_deep' | 'add_buffer' | 'adjust_break';
	title: string;
	description: string;
	data_source: string;
	confidence_score: number;
	is_applied: number;
	is_dismissed: number;
	created_at: string;
	applied_at: string | null;
	dismissed_at: string | null;
}
