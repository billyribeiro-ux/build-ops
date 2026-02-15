export interface QuizQuestion {
	id: string;
	day_plan_id: string;
	question_text: string;
	question_type: 'short_answer' | 'multiple_choice' | 'code_prompt' | 'reflection';
	correct_answer: string;
	options: string;
	points: number;
	time_limit_seconds: number;
	order_index: number;
	created_at: string;
}

export interface CreateQuizQuestionInput {
	day_plan_id: string;
	question_text: string;
	question_type: string;
	correct_answer: string;
	options?: string;
	points: number;
	time_limit_seconds: number;
}

export interface UpdateQuizQuestionInput {
	question_text?: string;
	question_type?: string;
	correct_answer?: string;
	options?: string;
	points?: number;
	time_limit_seconds?: number;
}

export interface QuizAttempt {
	id: string;
	day_attempt_id: string;
	quiz_question_id: string;
	answer: string;
	is_correct: boolean;
	time_taken_seconds: number;
	submitted_at: string;
}
