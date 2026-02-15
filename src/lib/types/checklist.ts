export interface ChecklistItem {
	id: string;
	day_plan_id: string;
	label: string;
	is_required: boolean;
	order_index: number;
	created_at: string;
}

export interface CreateChecklistItemInput {
	day_plan_id: string;
	label: string;
	is_required: boolean;
}

export interface UpdateChecklistItemInput {
	label?: string;
	is_required?: boolean;
}

export interface AttemptChecklist {
	id: string;
	day_attempt_id: string;
	checklist_item_id: string;
	is_completed: boolean;
	completed_at: string | null;
}
