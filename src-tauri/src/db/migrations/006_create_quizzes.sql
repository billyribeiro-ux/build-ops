CREATE TABLE IF NOT EXISTS quiz_questions (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL DEFAULT 'short_answer' CHECK (question_type IN ('short_answer', 'multiple_choice', 'code_prompt', 'reflection')),
    correct_answer TEXT NOT NULL DEFAULT '',
    options_json TEXT NOT NULL DEFAULT '[]',
    order_index INTEGER NOT NULL DEFAULT 0,
    points INTEGER NOT NULL DEFAULT 1,
    time_limit_seconds INTEGER NOT NULL DEFAULT 120,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES quiz_questions(id) ON DELETE CASCADE,
    user_answer TEXT NOT NULL DEFAULT '',
    is_correct INTEGER,
    score INTEGER NOT NULL DEFAULT 0,
    time_taken_seconds INTEGER NOT NULL DEFAULT 0,
    answered_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(attempt_id, question_id)
);

CREATE INDEX idx_quiz_questions_plan ON quiz_questions(day_plan_id);
CREATE INDEX idx_quiz_attempts_attempt ON quiz_attempts(attempt_id);
