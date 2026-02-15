CREATE TABLE IF NOT EXISTS day_dependencies (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    depends_on_day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    dependency_type TEXT NOT NULL DEFAULT 'prerequisite' CHECK (dependency_type IN ('prerequisite', 'recommended', 'related')),
    minimum_score INTEGER NOT NULL DEFAULT 70,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(day_plan_id, depends_on_day_plan_id),
    CHECK(day_plan_id != depends_on_day_plan_id)
);

CREATE INDEX idx_day_dependencies_plan ON day_dependencies(day_plan_id);
CREATE INDEX idx_day_dependencies_depends ON day_dependencies(depends_on_day_plan_id);
