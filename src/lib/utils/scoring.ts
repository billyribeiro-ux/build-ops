export const SCORE_WEIGHTS = {
	implementation: 40,
	codeQuality: 20,
	accessibility: 15,
	performance: 15,
	quiz: 10
} as const;

export const SCORE_THRESHOLDS = {
	mastery: 95,
	strongPass: 85,
	pass: 70,
	blocked: 0
} as const;

export type ScoreGrade = 'mastery' | 'strong_pass' | 'pass' | 'blocked';

export function getGrade(totalScore: number): ScoreGrade {
	if (totalScore >= SCORE_THRESHOLDS.mastery) return 'mastery';
	if (totalScore >= SCORE_THRESHOLDS.strongPass) return 'strong_pass';
	if (totalScore >= SCORE_THRESHOLDS.pass) return 'pass';
	return 'blocked';
}

export function getGradeLabel(grade: ScoreGrade): string {
	switch (grade) {
		case 'mastery': return 'Mastery';
		case 'strong_pass': return 'Strong Pass';
		case 'pass': return 'Pass';
		case 'blocked': return 'Blocked';
	}
}

export function getGradeColor(grade: ScoreGrade): string {
	switch (grade) {
		case 'mastery': return 'text-purple-400';
		case 'strong_pass': return 'text-green-400';
		case 'pass': return 'text-blue-400';
		case 'blocked': return 'text-red-400';
	}
}

export function getGradeBgColor(grade: ScoreGrade): string {
	switch (grade) {
		case 'mastery': return 'bg-purple-500/20';
		case 'strong_pass': return 'bg-green-500/20';
		case 'pass': return 'bg-blue-500/20';
		case 'blocked': return 'bg-red-500/20';
	}
}

export function calculateTotalScore(scores: {
	implementation: number;
	codeQuality: number;
	accessibility: number;
	performance: number;
	quiz: number;
}): number {
	return scores.implementation + scores.codeQuality + scores.accessibility + scores.performance + scores.quiz;
}

export function isBlocked(totalScore: number, memoryRebuildPassed: boolean): boolean {
	if (!memoryRebuildPassed) return true;
	return totalScore < SCORE_THRESHOLDS.pass;
}
