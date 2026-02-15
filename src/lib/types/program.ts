export interface Program {
	id: string;
	title: string;
	description: string;
	target_days: number;
	status: 'active' | 'paused' | 'completed' | 'archived';
	created_at: string;
	updated_at: string;
}

export interface CreateProgramInput {
	title: string;
	description: string;
	target_days: number;
}

export interface UpdateProgramInput {
	title?: string;
	description?: string;
	status?: string;
}
