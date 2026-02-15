import { invoke } from '@tauri-apps/api/core';
import type { Module, CreateModuleInput, UpdateModuleInput } from '$lib/types';

export async function createModule(input: CreateModuleInput): Promise<Module> {
  return invoke<Module>('create_module', { input });
}

export async function getModule(id: string): Promise<Module> {
  return invoke<Module>('get_module', { id });
}

export async function listModules(programId: string): Promise<Module[]> {
  return invoke<Module[]>('list_modules', { programId });
}

export async function updateModule(id: string, input: UpdateModuleInput): Promise<Module> {
  return invoke<Module>('update_module', { id, input });
}

export async function deleteModule(id: string): Promise<void> {
  return invoke<void>('delete_module', { id });
}

export async function reorderModules(programId: string, moduleIds: string[]): Promise<void> {
  return invoke<void>('reorder_modules', { programId, moduleIds });
}
