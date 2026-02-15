import { invoke } from '@tauri-apps/api/core';
import type {
	Program,
	CreateProgramInput,
	UpdateProgramInput,
	DayPlan,
	CreateDayPlanInput,
	DayAttempt,
	UpdateAttemptInput,
	UserCapacityProfile,
	UpdateCapacityInput,
	DaySession,
	CreateSessionInput,
	GeneratedPlan,
	TimeRecommendation,
	TimeAnalytics,
	FocusMetricsDaily
} from '$lib/types';

export const programCommands = {
	create: (input: CreateProgramInput) => invoke<Program>('create_program', { input }),
	get: (id: string) => invoke<Program>('get_program', { id }),
	list: () => invoke<Program[]>('list_programs'),
	update: (id: string, input: UpdateProgramInput) =>
		invoke<Program>('update_program', { id, input }),
	delete: (id: string) => invoke<void>('delete_program', { id })
};

export const dayPlanCommands = {
	create: (input: CreateDayPlanInput) => invoke<DayPlan>('create_day_plan', { input }),
	get: (id: string) => invoke<DayPlan>('get_day_plan', { id }),
	list: (programId: string) => invoke<DayPlan[]>('list_day_plans', { programId })
};

export const attemptCommands = {
	start: (dayPlanId: string) => invoke<DayAttempt>('start_attempt', { dayPlanId }),
	get: (id: string) => invoke<DayAttempt>('get_attempt', { id }),
	update: (id: string, input: UpdateAttemptInput) =>
		invoke<DayAttempt>('update_attempt', { id, input }),
	list: (dayPlanId: string) => invoke<DayAttempt[]>('list_attempts', { dayPlanId })
};

export const capacityCommands = {
	get: () => invoke<UserCapacityProfile>('get_capacity_profile'),
	update: (input: UpdateCapacityInput) =>
		invoke<UserCapacityProfile>('update_capacity_profile', { input })
};

export const sessionCommands = {
	create: (input: CreateSessionInput) => invoke<DaySession>('create_session', { input }),
	start: (id: string) => invoke<DaySession>('start_session', { id }),
	pause: (id: string) => invoke<DaySession>('pause_session', { id }),
	complete: (id: string, notes?: string) => invoke<DaySession>('complete_session', { id, notes }),
	list: (dayAttemptId: string) => invoke<DaySession[]>('list_sessions', { dayAttemptId })
};

export const timePlanningCommands = {
	planMyDay: (dayPlanId: string, dayAttemptId: string) =>
		invoke<GeneratedPlan>('plan_my_day', { dayPlanId, dayAttemptId })
};

export const recommendationCommands = {
	generate: () => invoke<TimeRecommendation[]>('generate_recommendations'),
	list: () => invoke<TimeRecommendation[]>('list_recommendations'),
	apply: (id: string) => invoke<TimeRecommendation>('apply_recommendation', { id }),
	dismiss: (id: string) => invoke<TimeRecommendation>('dismiss_recommendation', { id })
};

export const analyticsCommands = {
	getTimeAnalytics: () => invoke<TimeAnalytics>('get_time_analytics'),
	updateDailyMetrics: (date: string) => invoke<FocusMetricsDaily>('update_daily_metrics', { date })
};
