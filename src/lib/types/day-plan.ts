import type { ChecklistItem } from './checklist';
import type { QuizQuestion } from './quiz';
import type { ConceptTag } from './concept-tag';
import type { DayDependency } from './dependency';

export interface DayPlan {
	id: string;
	program_id: string;
	module_id: string;
	title: string;
	day_number: number;
	version: number;
	status: 'draft' | 'published' | 'archived';
	syntax_targets: string;
	implementation_brief: string;
	files_to_create: string;
	success_criteria: string;
	stretch_challenge: string;
	notes: string;
	estimated_minutes: number;
	memory_rebuild_minutes: number;
	min_minutes: number;
	recommended_minutes: number;
	deep_minutes: number;
	complexity_level: number;
	focus_blocks: string;
	created_at: string;
	updated_at: string;
}

export interface FocusBlock {
	session_type: string;
	minutes: number;
}

export interface DayPlanFull {
	day_plan: DayPlan;
	checklist_items: ChecklistItem[];
	quiz_questions: QuizQuestion[];
	concept_tags: ConceptTag[];
	dependencies: DayDependency[];
	module_title: string;
	module_color: string;
}

export interface DayPlanSummary {
	id: string;
	title: string;
	day_number: number;
	module_id: string;
	module_title: string;
	module_color: string;
	status: string;
	estimated_minutes: number;
	checklist_count: number;
	quiz_count: number;
	tag_count: number;
	best_score: number | null;
	attempt_count: number;
}

export interface CreateDayPlanInput {
	program_id: string;
	module_id: string;
	title: string;
	day_number: number;
	syntax_targets: string;
	implementation_brief: string;
	min_minutes?: number;
	recommended_minutes?: number;
	deep_minutes?: number;
	complexity_level?: number;
}

export interface UpdateDayPlanInput {
	title?: string;
	syntax_targets?: string;
	implementation_brief?: string;
	files_to_create?: string;
	success_criteria?: string;
	stretch_challenge?: string;
	notes?: string;
	status?: string;
	estimated_minutes?: number;
	memory_rebuild_minutes?: number;
}
