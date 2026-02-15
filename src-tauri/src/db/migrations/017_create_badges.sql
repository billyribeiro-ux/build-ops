CREATE TABLE IF NOT EXISTS badges (
    id TEXT PRIMARY KEY NOT NULL,
    badge_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL DEFAULT 'ph:trophy-bold',
    earned_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    program_id TEXT REFERENCES programs(id) ON DELETE CASCADE,
    day_plan_id TEXT REFERENCES day_plans(id) ON DELETE SET NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_badges_type ON badges(badge_type);
CREATE INDEX idx_badges_program ON badges(program_id);
