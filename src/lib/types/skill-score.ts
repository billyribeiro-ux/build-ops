export interface SkillScore {
	id: string;
	program_id: string;
	domain: string;
	score: number;
	total_attempts: number;
	last_updated: string;
	created_at: string;
}

export interface SkillRadarData {
	domain: string;
	score: number;
	max_score: number;
}
