export interface Artifact {
	id: string;
	day_attempt_id: string;
	artifact_type: 'file' | 'link' | 'code' | 'note';
	title: string;
	content: string;
	file_path: string | null;
	file_size: number | null;
	mime_type: string | null;
	url: string | null;
	created_at: string;
}

export interface CreateArtifactInput {
	day_attempt_id: string;
	artifact_type: string;
	title: string;
	content: string;
	file_path?: string;
	file_size?: number;
	mime_type?: string;
	url?: string;
}

export interface UpdateArtifactInput {
	title?: string;
	content?: string;
}
