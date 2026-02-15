export interface Badge {
	id: string;
	program_id: string;
	badge_type: string;
	title: string;
	description: string;
	icon: string;
	earned_at: string;
	metadata: string;
}

export interface BadgeProgress {
	badge_type: string;
	title: string;
	description: string;
	current_value: number;
	target_value: number;
	progress_percent: number;
	is_earned: boolean;
}
