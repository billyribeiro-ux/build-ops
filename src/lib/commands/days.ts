import { invoke } from '@tauri-apps/api/core';
import type {
	DayPlan,
	DayPlanFull,
	DayPlanSummary,
	CreateDayPlanInput,
	UpdateDayPlanInput,
	ChecklistItem,
	CreateChecklistItemInput,
	UpdateChecklistItemInput,
	QuizQuestion,
	CreateQuizQuestionInput,
	UpdateQuizQuestionInput,
	ConceptTag,
	CreateConceptTagInput,
	DayDependency,
	CreateDependencyInput,
	DependencyStatus
} from '$lib/types';

// Core Day Plan CRUD
export async function createDayPlan(input: CreateDayPlanInput): Promise<DayPlan> {
	return invoke<DayPlan>('create_day_plan', { input });
}

export async function getDayPlan(id: string): Promise<DayPlanFull> {
	return invoke<DayPlanFull>('get_day_plan', { id });
}

export async function listDayPlans(programId: string): Promise<DayPlanSummary[]> {
	return invoke<DayPlanSummary[]>('list_day_plans', { programId });
}

export async function listDayPlansByModule(moduleId: string): Promise<DayPlanSummary[]> {
	return invoke<DayPlanSummary[]>('list_day_plans_by_module', { moduleId });
}

export async function updateDayPlan(id: string, input: UpdateDayPlanInput): Promise<DayPlan> {
	return invoke<DayPlan>('update_day_plan', { id, input });
}

export async function deleteDayPlan(id: string): Promise<void> {
	return invoke<void>('delete_day_plan', { id });
}

export async function reorderDayPlans(dayPlanIds: string[]): Promise<void> {
	return invoke<void>('reorder_day_plans', { dayPlanIds });
}

export async function duplicateDayPlan(id: string): Promise<DayPlan> {
	return invoke<DayPlan>('duplicate_day_plan', { id });
}

// Checklist Management
export async function addChecklistItem(input: CreateChecklistItemInput): Promise<ChecklistItem> {
	return invoke<ChecklistItem>('add_checklist_item', { input });
}

export async function updateChecklistItem(
	id: string,
	input: UpdateChecklistItemInput
): Promise<ChecklistItem> {
	return invoke<ChecklistItem>('update_checklist_item', { id, input });
}

export async function deleteChecklistItem(id: string): Promise<void> {
	return invoke<void>('delete_checklist_item', { id });
}

export async function reorderChecklistItems(itemIds: string[]): Promise<void> {
	return invoke<void>('reorder_checklist_items', { itemIds });
}

// Quiz Management
export async function addQuizQuestion(input: CreateQuizQuestionInput): Promise<QuizQuestion> {
	return invoke<QuizQuestion>('add_quiz_question', { input });
}

export async function updateQuizQuestion(
	id: string,
	input: UpdateQuizQuestionInput
): Promise<QuizQuestion> {
	return invoke<QuizQuestion>('update_quiz_question', { id, input });
}

export async function deleteQuizQuestion(id: string): Promise<void> {
	return invoke<void>('delete_quiz_question', { id });
}

// Concept Tags
export async function createConceptTag(input: CreateConceptTagInput): Promise<ConceptTag> {
	return invoke<ConceptTag>('create_concept_tag', { input });
}

export async function listConceptTags(): Promise<ConceptTag[]> {
	return invoke<ConceptTag[]>('list_concept_tags');
}

export async function addTagToDay(dayPlanId: string, conceptTagId: string): Promise<void> {
	return invoke<void>('add_tag_to_day', { dayPlanId, conceptTagId });
}

export async function removeTagFromDay(dayPlanId: string, conceptTagId: string): Promise<void> {
	return invoke<void>('remove_tag_from_day', { dayPlanId, conceptTagId });
}

// Dependencies
export async function addDependency(input: CreateDependencyInput): Promise<DayDependency> {
	return invoke<DayDependency>('add_dependency', { input });
}

export async function removeDependency(id: string): Promise<void> {
	return invoke<void>('remove_dependency', { id });
}

export async function checkDependencies(dayPlanId: string): Promise<DependencyStatus[]> {
	return invoke<DependencyStatus[]>('check_dependencies', { dayPlanId });
}
