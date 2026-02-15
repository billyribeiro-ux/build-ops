/**
 * SM-2 Spaced Repetition Algorithm
 * Based on SuperMemo 2 by Piotr Wozniak
 */

export interface ReviewCard {
	conceptTag: string;
	easeFactor: number;
	interval: number;
	repetitions: number;
	nextReviewDate: string;
	lastReviewDate: string;
}

export interface ReviewResult {
	quality: number; // 0-5 scale
}

export function calculateNextReview(card: ReviewCard, result: ReviewResult): ReviewCard {
	const { quality } = result;
	let { easeFactor, interval, repetitions } = card;

	if (quality < 3) {
		repetitions = 0;
		interval = 1;
	} else {
		if (repetitions === 0) {
			interval = 1;
		} else if (repetitions === 1) {
			interval = 6;
		} else {
			interval = Math.round(interval * easeFactor);
		}
		repetitions += 1;
	}

	easeFactor = Math.max(
		1.3,
		easeFactor + (0.1 - (5 - quality) * (0.08 + (5 - quality) * 0.02))
	);

	const now = new Date();
	const nextDate = new Date(now.getTime() + interval * 24 * 60 * 60 * 1000);

	return {
		...card,
		easeFactor,
		interval,
		repetitions,
		lastReviewDate: now.toISOString(),
		nextReviewDate: nextDate.toISOString()
	};
}

export function createNewCard(conceptTag: string): ReviewCard {
	return {
		conceptTag,
		easeFactor: 2.5,
		interval: 0,
		repetitions: 0,
		nextReviewDate: new Date().toISOString(),
		lastReviewDate: new Date().toISOString()
	};
}

export function isDueForReview(card: ReviewCard): boolean {
	return new Date(card.nextReviewDate) <= new Date();
}

export function qualityFromScore(score: number, maxScore: number): number {
	const ratio = score / maxScore;
	if (ratio >= 0.95) return 5;
	if (ratio >= 0.85) return 4;
	if (ratio >= 0.70) return 3;
	if (ratio >= 0.50) return 2;
	if (ratio >= 0.30) return 1;
	return 0;
}
