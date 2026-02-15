export interface DayDependency {
	id: string;
	day_plan_id: string;
	depends_on_day_plan_id: string;
	dependency_type: 'prerequisite' | 'recommended' | 'related';
	minimum_score: number;
	created_at: string;
}

export interface CreateDependencyInput {
	day_plan_id: string;
	depends_on_day_plan_id: string;
	dependency_type: string;
	minimum_score: number;
}

export interface DependencyStatus {
	dependency_id: string;
	depends_on_day_plan_id: string;
	depends_on_title: string;
	depends_on_day_number: number;
	dependency_type: string;
	minimum_score: number;
	is_met: boolean;
	best_score: number | null;
	best_attempt_status: string | null;
}
