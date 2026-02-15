export interface ImportJob {
  id: string;
  program_id: string | null;
  status: ImportStatus;
  source_type: SourceType;
  source_files_json: string;
  extracted_text: string;
  extracted_sections_json: string;
  ai_analysis_json: string;
  generated_plan_json: string;
  reviewed_plan_json: string | null;
  total_pages: number;
  total_tokens: number;
  total_days_generated: number;
  ai_model_used: string;
  error_message: string | null;
  error_step: string | null;
  started_at: string | null;
  completed_at: string | null;
  created_at: string;
  updated_at: string;
}

export type ImportStatus = 
  | 'pending' 
  | 'extracting' 
  | 'analyzing' 
  | 'generating' 
  | 'review' 
  | 'applying' 
  | 'completed' 
  | 'failed';

export type SourceType = 'pdf' | 'markdown' | 'text' | 'multi_file';

export interface ImportJobSummary {
  id: string;
  program_id: string | null;
  status: ImportStatus;
  source_type: SourceType;
  total_days_generated: number;
  created_at: string;
}

export interface SourceFile {
  file_name: string;
  file_path: string;
  file_size: number;
}

export interface ImportGeneratedPlan {
  program: ProgramDraft;
  modules: ModuleDraft[];
  day_plans: DayPlanDraft[];
  checklist_items: ChecklistItemDraft[];
  quiz_questions: QuizQuestionDraft[];
  concept_tags: ConceptTagDraft[];
  tag_assignments: [number, string][];
  dependencies: DependencyDraft[];
  validation_warnings: string[];
}

export interface ProgramDraft {
  title: string;
  description: string;
  estimated_total_days: number;
}

export interface ModuleDraft {
  title: string;
  description: string;
  order_index: number;
  color: string;
}

export interface DayPlanDraft {
  module_index: number;
  day_number: number;
  title: string;
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
}

export interface ChecklistItemDraft {
  day_index: number;
  label: string;
  is_required: boolean;
  order_index: number;
}

export interface QuizQuestionDraft {
  day_index: number;
  question_text: string;
  question_type: 'short_answer' | 'multiple_choice' | 'code_prompt' | 'reflection';
  correct_answer: string;
  options: string[];
  points: number;
  time_limit_seconds: number;
}

export interface ConceptTagDraft {
  name: string;
  domain: string;
}

export interface DependencyDraft {
  day_index: number;
  depends_on_day_number: number;
  dependency_type: 'prerequisite' | 'recommended';
  minimum_score: number;
}
