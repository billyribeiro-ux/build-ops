CREATE TABLE IF NOT EXISTS checklist_items (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS attempt_checklist (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    checklist_item_id TEXT NOT NULL REFERENCES checklist_items(id) ON DELETE CASCADE,
    is_completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    UNIQUE(attempt_id, checklist_item_id)
);

CREATE INDEX idx_checklist_items_plan ON checklist_items(day_plan_id);
CREATE INDEX idx_attempt_checklist_attempt ON attempt_checklist(attempt_id);
