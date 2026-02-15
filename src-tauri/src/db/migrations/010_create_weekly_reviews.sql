CREATE TABLE IF NOT EXISTS weekly_reviews (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    week_number INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    days_completed INTEGER NOT NULL DEFAULT 0,
    days_blocked INTEGER NOT NULL DEFAULT 0,
    average_score REAL NOT NULL DEFAULT 0.0,
    best_score INTEGER NOT NULL DEFAULT 0,
    worst_score INTEGER NOT NULL DEFAULT 0,
    total_time_minutes INTEGER NOT NULL DEFAULT 0,
    strong_concepts_json TEXT NOT NULL DEFAULT '[]',
    weak_concepts_json TEXT NOT NULL DEFAULT '[]',
    replay_recommendations_json TEXT NOT NULL DEFAULT '[]',
    summary TEXT NOT NULL DEFAULT '',
    goals_next_week TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, week_number)
);

CREATE INDEX idx_weekly_reviews_program ON weekly_reviews(program_id);
