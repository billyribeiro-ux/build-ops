import { invoke } from '@tauri-apps/api/core';
import type {
	SpacedRepetition,
	RecordReviewInput,
	DueReview,
	Streak,
	Badge,
	SkillRadarData
} from '$lib/types';

// Spaced Repetition Commands
export async function getDueReviews(programId: string): Promise<DueReview[]> {
	return invoke<DueReview[]>('get_due_reviews', { programId });
}

export async function recordReview(input: RecordReviewInput): Promise<SpacedRepetition> {
	return invoke<SpacedRepetition>('record_review', { input });
}

export async function getForgettingCurveAlerts(programId: string): Promise<DueReview[]> {
	return invoke<DueReview[]>('get_forgetting_curve_alerts', { programId });
}

// Streak Commands
export async function getStreak(programId: string): Promise<Streak> {
	return invoke<Streak>('get_streak', { programId });
}

export async function updateStreak(programId: string): Promise<Streak> {
	return invoke<Streak>('update_streak', { programId });
}

export async function useStreakFreeze(programId: string, reason: string): Promise<Streak> {
	return invoke<Streak>('use_streak_freeze', { programId, reason });
}

// Badge Commands
export async function getBadges(programId: string): Promise<Badge[]> {
	return invoke<Badge[]>('get_badges', { programId });
}

export async function checkAndAwardBadges(programId: string): Promise<Badge[]> {
	return invoke<Badge[]>('check_and_award_badges', { programId });
}

// Skill Score Commands
export async function getSkillRadar(programId: string): Promise<SkillRadarData[]> {
	return invoke<SkillRadarData[]>('get_skill_radar', { programId });
}

export async function updateSkillScores(programId: string, dayAttemptId: string): Promise<void> {
	return invoke<void>('update_skill_scores', { programId, dayAttemptId });
}
