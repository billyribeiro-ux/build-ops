CREATE TABLE IF NOT EXISTS streaks (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    current_streak INTEGER NOT NULL DEFAULT 0,
    longest_streak INTEGER NOT NULL DEFAULT 0,
    last_active_date TEXT,
    streak_freezes_used INTEGER NOT NULL DEFAULT 0,
    streak_freezes_available INTEGER NOT NULL DEFAULT 2,
    freeze_reset_month TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id)
);
