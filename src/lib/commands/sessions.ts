import { invoke } from '@tauri-apps/api/core';
import type { DaySession, CreateSessionInput } from '$lib/types';

export async function createSession(input: CreateSessionInput): Promise<DaySession> {
	return invoke<DaySession>('create_session', { input });
}

export async function startSession(id: string): Promise<DaySession> {
	return invoke<DaySession>('start_session', { id });
}

export async function pauseSession(id: string): Promise<DaySession> {
	return invoke<DaySession>('pause_session', { id });
}

export async function completeSession(id: string, notes?: string): Promise<DaySession> {
	return invoke<DaySession>('complete_session', { id, notes });
}

export async function listSessions(dayAttemptId: string): Promise<DaySession[]> {
	return invoke<DaySession[]>('list_sessions', { dayAttemptId });
}
