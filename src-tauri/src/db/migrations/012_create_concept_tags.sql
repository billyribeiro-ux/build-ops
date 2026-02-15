CREATE TABLE IF NOT EXISTS concept_tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    domain TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#6366F1',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS day_plan_tags (
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES concept_tags(id) ON DELETE CASCADE,
    PRIMARY KEY(day_plan_id, tag_id)
);

CREATE INDEX idx_concept_tags_domain ON concept_tags(domain);
CREATE INDEX idx_day_plan_tags_plan ON day_plan_tags(day_plan_id);
CREATE INDEX idx_day_plan_tags_tag ON day_plan_tags(tag_id);
