use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ImportJob {
    pub id: String,
    pub program_id: Option<String>,
    pub status: String,
    pub source_type: String,
    pub source_files_json: String,
    pub extracted_text: String,
    pub extracted_sections_json: String,
    pub ai_analysis_json: String,
    pub generated_plan_json: String,
    pub reviewed_plan_json: Option<String>,
    pub total_pages: i64,
    pub total_tokens: i64,
    pub total_days_generated: i64,
    pub ai_model_used: String,
    pub error_message: Option<String>,
    pub error_step: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub file_name: String,
    pub file_path: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedDocument {
    pub file_name: String,
    pub total_pages: usize,
    pub raw_text: String,
    pub sections: Vec<ExtractedSection>,
    pub code_blocks: Vec<CodeBlock>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedSection {
    pub heading: String,
    pub level: u8,
    pub content: String,
    pub page_number: usize,
    pub has_code: bool,
    pub has_list: bool,
    pub estimated_complexity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
    pub context_heading: String,
    pub page_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub page_count: usize,
    pub word_count: usize,
    pub detected_languages: Vec<String>,
    pub detected_topics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkedDocument {
    pub chunks: Vec<DocumentChunk>,
    pub total_tokens: usize,
    pub chunk_strategy: ChunkStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub index: usize,
    pub content: String,
    pub token_count: usize,
    pub section_refs: Vec<usize>,
    pub is_continuation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkStrategy {
    SinglePass,
    SectionBased,
    MultiPass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPlan {
    pub program: ProgramDraft,
    pub modules: Vec<ModuleDraft>,
    pub day_plans: Vec<DayPlanDraft>,
    pub checklist_items: Vec<ChecklistItemDraft>,
    pub quiz_questions: Vec<QuizQuestionDraft>,
    pub concept_tags: Vec<ConceptTagDraft>,
    pub tag_assignments: Vec<(usize, String)>,
    pub dependencies: Vec<DependencyDraft>,
    pub validation_warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDraft {
    pub title: String,
    pub description: String,
    pub estimated_total_days: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDraft {
    pub title: String,
    pub description: String,
    pub order_index: i64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlanDraft {
    pub module_index: usize,
    pub day_number: i64,
    pub title: String,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub files_to_create: String,
    pub success_criteria: String,
    pub stretch_challenge: String,
    pub notes: String,
    pub estimated_minutes: i64,
    pub memory_rebuild_minutes: i64,
    pub min_minutes: i64,
    pub recommended_minutes: i64,
    pub deep_minutes: i64,
    pub complexity_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItemDraft {
    pub day_index: usize,
    pub label: String,
    pub is_required: bool,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestionDraft {
    pub day_index: usize,
    pub question_text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub points: i64,
    pub time_limit_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptTagDraft {
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyDraft {
    pub day_index: usize,
    pub depends_on_day_number: i64,
    pub dependency_type: String,
    pub minimum_score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ImportJobSummary {
    pub id: String,
    pub program_id: Option<String>,
    pub status: String,
    pub source_type: String,
    pub total_days_generated: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResponse {
    pub program_title: String,
    pub program_description: String,
    pub estimated_total_days: i64,
    pub modules: Vec<AiModuleResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModuleResponse {
    pub title: String,
    pub description: String,
    pub order_index: i64,
    pub color: String,
    pub days: Vec<AiDayResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiDayResponse {
    pub day_number: i64,
    pub title: String,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub files_to_create: String,
    pub success_criteria: String,
    pub stretch_challenge: String,
    pub notes: String,
    pub estimated_minutes: i64,
    pub memory_rebuild_minutes: i64,
    pub checklist_items: Vec<AiChecklistItem>,
    pub quiz_questions: Vec<AiQuizQuestion>,
    pub concept_tags: Vec<AiConceptTag>,
    pub dependencies: Vec<AiDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChecklistItem {
    pub label: String,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiQuizQuestion {
    pub question_text: String,
    pub question_type: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub points: i64,
    pub time_limit_seconds: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConceptTag {
    pub name: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiDependency {
    pub depends_on_day_number: i64,
    #[serde(rename = "type")]
    pub dependency_type: String,
    pub minimum_score: i64,
}
