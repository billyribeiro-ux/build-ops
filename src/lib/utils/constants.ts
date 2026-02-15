export const APP_NAME = 'BuildOps 40';
export const APP_VERSION = '0.1.0';

export const AUTOSAVE_INTERVAL_MS = 5000;
export const DEFAULT_SESSION_MINUTES = 60;
export const DEFAULT_MEMORY_REBUILD_MINUTES = 15;
export const DEFAULT_QUIZ_TIME_LIMIT_SECONDS = 120;

export const SCORE_MAX = 100;
export const SCORE_IMPLEMENTATION_MAX = 40;
export const SCORE_CODE_QUALITY_MAX = 20;
export const SCORE_ACCESSIBILITY_MAX = 15;
export const SCORE_PERFORMANCE_MAX = 15;
export const SCORE_QUIZ_MAX = 10;

export const STREAK_FREEZES_PER_MONTH = 2;

export const STATUS_COLORS: Record<string, string> = {
	active: 'bg-green-500',
	paused: 'bg-yellow-500',
	completed: 'bg-blue-500',
	archived: 'bg-gray-500',
	draft: 'bg-gray-500',
	published: 'bg-indigo-500',
	in_progress: 'bg-amber-500',
	submitted: 'bg-blue-500',
	blocked: 'bg-red-500',
	passed: 'bg-green-500',
	mastery: 'bg-purple-500'
};

export const COMPLEXITY_LABELS: Record<number, string> = {
	1: 'Easy',
	2: 'Moderate',
	3: 'Challenging',
	4: 'Hard',
	5: 'Expert'
};

export const COMPLEXITY_COLORS: Record<number, string> = {
	1: 'text-green-400',
	2: 'text-blue-400',
	3: 'text-yellow-400',
	4: 'text-orange-400',
	5: 'text-red-400'
};
