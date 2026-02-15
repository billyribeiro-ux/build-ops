export interface ConceptTag {
	id: string;
	name: string;
	domain: string;
	color: string;
	created_at: string;
}

export interface CreateConceptTagInput {
	name: string;
	domain: string;
	color: string;
}
