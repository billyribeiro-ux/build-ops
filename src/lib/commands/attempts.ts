import { invoke } from '@tauri-apps/api/core';
import type {
	DayAttempt,
	CreateDayAttemptInput,
	UpdateDayAttemptInput,
	SubmitScoresInput,
	DayAttemptSummary,
	ExerciseEntry,
	CreateExerciseEntryInput,
	UpdateExerciseEntryInput,
	Artifact,
	CreateArtifactInput,
	UpdateArtifactInput,
	BugLog,
	CreateBugLogInput,
	UpdateBugLogInput
} from '$lib/types';

// Day Attempt Commands
export async function startAttempt(input: CreateDayAttemptInput): Promise<DayAttempt> {
	return invoke<DayAttempt>('start_attempt', { input });
}

export async function getAttempt(id: string): Promise<DayAttempt> {
	return invoke<DayAttempt>('get_attempt', { id });
}

export async function getCurrentAttempt(dayPlanId: string): Promise<DayAttempt | null> {
	return invoke<DayAttempt | null>('get_current_attempt', { dayPlanId });
}

export async function listAttempts(dayPlanId: string): Promise<DayAttemptSummary[]> {
	return invoke<DayAttemptSummary[]>('list_attempts', { dayPlanId });
}

export async function updateAttempt(id: string, input: UpdateDayAttemptInput): Promise<DayAttempt> {
	return invoke<DayAttempt>('update_attempt', { id, input });
}

export async function autosaveAttempt(id: string): Promise<void> {
	return invoke<void>('autosave_attempt', { id });
}

export async function submitAttempt(id: string, input: SubmitScoresInput): Promise<DayAttempt> {
	return invoke<DayAttempt>('submit_attempt', { id, input });
}

export async function deleteAttempt(id: string): Promise<void> {
	return invoke<void>('delete_attempt', { id });
}

// Exercise Entry Commands
export async function createExerciseEntry(input: CreateExerciseEntryInput): Promise<ExerciseEntry> {
	return invoke<ExerciseEntry>('create_exercise_entry', { input });
}

export async function updateExerciseEntry(
	id: string,
	input: UpdateExerciseEntryInput
): Promise<ExerciseEntry> {
	return invoke<ExerciseEntry>('update_exercise_entry', { id, input });
}

export async function listExerciseEntries(dayAttemptId: string): Promise<ExerciseEntry[]> {
	return invoke<ExerciseEntry[]>('list_exercise_entries', { dayAttemptId });
}

export async function deleteExerciseEntry(id: string): Promise<void> {
	return invoke<void>('delete_exercise_entry', { id });
}

// Artifact Commands
export async function createArtifact(input: CreateArtifactInput): Promise<Artifact> {
	return invoke<Artifact>('create_artifact', { input });
}

export async function updateArtifact(id: string, input: UpdateArtifactInput): Promise<Artifact> {
	return invoke<Artifact>('update_artifact', { id, input });
}

export async function listArtifacts(dayAttemptId: string): Promise<Artifact[]> {
	return invoke<Artifact[]>('list_artifacts', { dayAttemptId });
}

export async function deleteArtifact(id: string): Promise<void> {
	return invoke<void>('delete_artifact', { id });
}

// Bug Log Commands
export async function createBugLog(input: CreateBugLogInput): Promise<BugLog> {
	return invoke<BugLog>('create_bug_log', { input });
}

export async function updateBugLog(id: string, input: UpdateBugLogInput): Promise<BugLog> {
	return invoke<BugLog>('update_bug_log', { id, input });
}

export async function listBugLogs(dayAttemptId: string): Promise<BugLog[]> {
	return invoke<BugLog[]>('list_bug_logs', { dayAttemptId });
}

export async function deleteBugLog(id: string): Promise<void> {
	return invoke<void>('delete_bug_log', { id });
}
