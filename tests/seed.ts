import Database from 'better-sqlite3';
import { randomUUID } from 'crypto';

export interface SeedData {
	programId: string;
	moduleId: string;
	dayPlanId: string;
	attemptId: string;
}

export function seedTestDatabase(dbPath: string): SeedData {
	const db = new Database(dbPath);
	
	// Enable foreign keys
	db.exec('PRAGMA foreign_keys = ON');
	
	const programId = randomUUID();
	const moduleId = randomUUID();
	const dayPlanId = randomUUID();
	const attemptId = randomUUID();
	const checklistId1 = randomUUID();
	const checklistId2 = randomUUID();
	const quizId1 = randomUUID();
	const quizId2 = randomUUID();
	
	// Create program
	db.prepare(`
		INSERT INTO programs (id, title, description, status, created_at, updated_at)
		VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))
	`).run(programId, 'React Mastery E2E Test', 'End-to-end test program', 'active');
	
	// Create module
	db.prepare(`
		INSERT INTO modules (id, program_id, title, description, order_index, color, created_at, updated_at)
		VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
	`).run(moduleId, programId, 'Fundamentals', 'React basics', 0, '#3b82f6');
	
	// Create day plan
	db.prepare(`
		INSERT INTO day_plans (
			id, program_id, module_id, title, day_number, version, status,
			syntax_targets, implementation_brief, files_to_create, success_criteria,
			estimated_minutes, min_minutes, recommended_minutes, deep_minutes,
			complexity_level, created_at, updated_at
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
	`).run(
		dayPlanId, programId, moduleId, 'Build a Counter Component', 1, 1, 'published',
		'useState, onClick, JSX', 'Create a counter with increment/decrement buttons',
		'Counter.jsx, Counter.css', 'Counter increments and decrements correctly',
		60, 30, 60, 90, 2
	);
	
	// Create checklist items
	db.prepare(`
		INSERT INTO checklist_items (id, day_plan_id, label, order_index, created_at)
		VALUES (?, ?, ?, ?, datetime('now'))
	`).run(checklistId1, dayPlanId, 'Create Counter component file', 0);
	
	db.prepare(`
		INSERT INTO checklist_items (id, day_plan_id, label, order_index, created_at)
		VALUES (?, ?, ?, ?, datetime('now'))
	`).run(checklistId2, dayPlanId, 'Add increment and decrement buttons', 1);
	
	// Create quiz questions
	db.prepare(`
		INSERT INTO quiz_questions (
			id, day_plan_id, question_text, correct_answer, points, order_index, created_at
		) VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
	`).run(quizId1, dayPlanId, 'What hook manages state in React?', 'useState', 5, 0);
	
	db.prepare(`
		INSERT INTO quiz_questions (
			id, day_plan_id, question_text, correct_answer, points, order_index, created_at
		) VALUES (?, ?, ?, ?, ?, ?, datetime('now'))
	`).run(quizId2, dayPlanId, 'What is JSX?', 'JavaScript XML syntax extension', 5, 1);
	
	// Create day attempt
	db.prepare(`
		INSERT INTO day_attempts (
			id, day_plan_id, day_plan_version, attempt_number, status,
			score_implementation, score_code_quality, score_accessibility, score_performance, score_quiz,
			memory_rebuild_completed, memory_rebuild_passed,
			started_at, actual_minutes, is_draft, created_at, updated_at
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), ?, ?, datetime('now'), datetime('now'))
	`).run(
		attemptId, dayPlanId, 1, 1, 'in_progress',
		0, 0, 0, 0, 0,
		0, 0,
		30, 1
	);
	
	// Create user capacity profile
	db.prepare(`
		INSERT OR REPLACE INTO user_capacity_profiles (
			id, user_id, default_daily_minutes, weekly_study_days,
			preferred_start_time, max_deep_days_per_week, break_pattern, timezone,
			created_at, updated_at
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
	`).run(
		randomUUID(), 'default', 180, 5,
		'09:00', 3, '25/5', 'America/New_York'
	);
	
	db.close();
	
	return { programId, moduleId, dayPlanId, attemptId };
}

export function cleanTestDatabase(dbPath: string) {
	const db = new Database(dbPath);
	
	// Delete all data in reverse dependency order
	const tables = [
		'session_interruptions',
		'day_sessions',
		'quiz_attempts',
		'attempt_checklist',
		'artifacts',
		'exercise_entries',
		'bug_logs',
		'day_attempts',
		'quiz_questions',
		'checklist_items',
		'concept_tags',
		'day_dependencies',
		'day_plans',
		'modules',
		'programs',
		'spaced_repetitions',
		'streaks',
		'badges',
		'skill_scores',
		'time_recommendations',
		'focus_metrics_daily',
		'import_jobs',
		'settings'
	];
	
	for (const table of tables) {
		try {
			db.prepare(`DELETE FROM ${table}`).run();
		} catch (err) {
			// Table might not exist, continue
		}
	}
	
	db.close();
}
