import { invoke } from '@tauri-apps/api/core';
import type { Program, ProgramSummary, ProgramStats, CreateProgramInput, UpdateProgramInput } from '$lib/types';

export async function createProgram(input: CreateProgramInput): Promise<Program> {
  return invoke<Program>('create_program', { input });
}

export async function getProgram(id: string): Promise<Program> {
  return invoke<Program>('get_program', { id });
}

export async function listPrograms(): Promise<ProgramSummary[]> {
  return invoke<ProgramSummary[]>('list_programs');
}

export async function updateProgram(id: string, input: UpdateProgramInput): Promise<Program> {
  return invoke<Program>('update_program', { id, input });
}

export async function deleteProgram(id: string): Promise<void> {
  return invoke<void>('delete_program', { id });
}

export async function duplicateProgram(id: string, newTitle: string): Promise<Program> {
  return invoke<Program>('duplicate_program', { id, newTitle });
}

export async function getProgramStats(id: string): Promise<ProgramStats> {
  return invoke<ProgramStats>('get_program_stats', { id });
}
