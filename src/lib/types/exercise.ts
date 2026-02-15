export interface ExerciseEntry {
	id: string;
	day_attempt_id: string;
	language: string;
	code: string;
	notes: string;
	order_index: number;
	created_at: string;
	updated_at: string;
}

export interface CreateExerciseEntryInput {
	day_attempt_id: string;
	language: string;
	code: string;
	notes: string;
}

export interface UpdateExerciseEntryInput {
	language?: string;
	code?: string;
	notes?: string;
}
