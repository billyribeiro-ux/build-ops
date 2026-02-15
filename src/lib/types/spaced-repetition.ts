export interface SpacedRepetition {
	id: string;
	day_plan_id: string;
	concept_tag_id: string;
	easiness_factor: number;
	interval_days: number;
	repetition_count: number;
	last_review_date: string;
	next_review_date: string;
	last_score: number;
	created_at: string;
	updated_at: string;
}

export interface RecordReviewInput {
	day_plan_id: string;
	concept_tag_id: string;
	score: number;
}

export interface DueReview {
	id: string;
	day_plan_id: string;
	day_title: string;
	day_number: number;
	concept_tag_id: string;
	concept_name: string;
	next_review_date: string;
	interval_days: number;
}
