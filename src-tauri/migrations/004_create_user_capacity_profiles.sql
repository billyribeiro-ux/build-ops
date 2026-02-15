CREATE TABLE IF NOT EXISTS user_capacity_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL DEFAULT 'default',
    default_daily_minutes INTEGER NOT NULL DEFAULT 180,
    weekly_study_days INTEGER NOT NULL DEFAULT 5,
    preferred_start_time TEXT NOT NULL DEFAULT '18:00',
    max_deep_days_per_week INTEGER NOT NULL DEFAULT 2,
    break_pattern TEXT NOT NULL DEFAULT '50/10',
    timezone TEXT NOT NULL DEFAULT 'UTC',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(user_id)
);

INSERT OR IGNORE INTO user_capacity_profiles (id, user_id) 
VALUES ('default-profile', 'default');

CREATE INDEX idx_capacity_user ON user_capacity_profiles(user_id);
