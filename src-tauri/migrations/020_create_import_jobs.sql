CREATE TABLE IF NOT EXISTS import_jobs (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT REFERENCES programs(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'extracting', 'analyzing', 'generating', 'review', 'applying', 'completed', 'failed')),
    source_type TEXT NOT NULL CHECK (source_type IN ('pdf', 'markdown', 'text', 'multi_file')),
    
    source_files_json TEXT NOT NULL DEFAULT '[]',
    
    extracted_text TEXT NOT NULL DEFAULT '',
    extracted_sections_json TEXT NOT NULL DEFAULT '[]',
    
    ai_analysis_json TEXT NOT NULL DEFAULT '{}',
    
    generated_plan_json TEXT NOT NULL DEFAULT '{}',
    
    reviewed_plan_json TEXT,
    
    total_pages INTEGER NOT NULL DEFAULT 0,
    total_tokens INTEGER NOT NULL DEFAULT 0,
    total_days_generated INTEGER NOT NULL DEFAULT 0,
    ai_model_used TEXT NOT NULL DEFAULT 'claude-sonnet-4-20250514',
    
    error_message TEXT,
    error_step TEXT,
    
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_import_jobs_status ON import_jobs(status);
CREATE INDEX idx_import_jobs_program ON import_jobs(program_id);
CREATE INDEX idx_import_jobs_created ON import_jobs(created_at DESC);
