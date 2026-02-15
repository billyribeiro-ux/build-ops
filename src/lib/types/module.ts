export interface Module {
	id: string;
	program_id: string;
	title: string;
	description: string;
	order_index: number;
	color: string;
	created_at: string;
	updated_at: string;
}

export interface CreateModuleInput {
	program_id: string;
	title: string;
	description: string;
	color: string;
}

export interface UpdateModuleInput {
	title?: string;
	description?: string;
	color?: string;
}
