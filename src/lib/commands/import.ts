import { invoke } from '@tauri-apps/api/core';
import type { 
  ImportJob, 
  ImportJobSummary, 
  ImportGeneratedPlan 
} from '$lib/types';
import type { Program } from '$lib/types';

export async function startImport(
  filePaths: string[],
  programId: string | null,
  apiKey: string
): Promise<ImportJob> {
  return invoke('start_import', { filePaths, programId, apiKey });
}

export async function getImportJob(jobId: string): Promise<ImportJob> {
  return invoke('get_import_job', { jobId });
}

export async function getImportPreview(jobId: string): Promise<ImportGeneratedPlan> {
  return invoke('get_import_preview', { jobId });
}

export async function updateImportPreview(
  jobId: string,
  reviewedPlanJson: string
): Promise<void> {
  return invoke('update_import_preview', { jobId, reviewedPlanJson });
}

export async function applyImport(jobId: string): Promise<Program> {
  return invoke('apply_import', { jobId });
}

export async function cancelImport(jobId: string): Promise<void> {
  return invoke('cancel_import', { jobId });
}

export async function listImportJobs(): Promise<ImportJobSummary[]> {
  return invoke('list_import_jobs');
}

export async function deleteImportJob(jobId: string): Promise<void> {
  return invoke('delete_import_job', { jobId });
}

export async function retryImport(jobId: string, apiKey: string): Promise<ImportJob> {
  return invoke('retry_import', { jobId, apiKey });
}
