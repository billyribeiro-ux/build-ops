<script lang="ts">
  import type { ImportGeneratedPlan } from '$lib/types';
  
  interface Props {
    plan: ImportGeneratedPlan;
    onSave: (updatedPlan: ImportGeneratedPlan) => void;
  }
  
  let { plan, onSave }: Props = $props();
  
  let editedPlan = $state<ImportGeneratedPlan>(JSON.parse(JSON.stringify(plan)));
  let selectedTab = $state<'overview' | 'modules' | 'days' | 'warnings'>('overview');
  let expandedModules = $state<Set<number>>(new Set([0]));
  
  function toggleModule(index: number) {
    const newSet = new Set(expandedModules);
    if (newSet.has(index)) {
      newSet.delete(index);
    } else {
      newSet.add(index);
    }
    expandedModules = newSet;
  }
  
  function handleSave() {
    onSave(editedPlan);
  }
  
  const daysByModule = $derived(() => {
    const grouped: Record<number, typeof editedPlan.day_plans> = {};
    editedPlan.day_plans.forEach(day => {
      if (!grouped[day.module_index]) {
        grouped[day.module_index] = [];
      }
      grouped[day.module_index].push(day);
    });
    return grouped;
  });
</script>

<div class="review-interface">
  <div class="tabs">
    <button 
      class="tab" 
      class:active={selectedTab === 'overview'}
      onclick={() => selectedTab = 'overview'}
    >
      Overview
    </button>
    <button 
      class="tab" 
      class:active={selectedTab === 'modules'}
      onclick={() => selectedTab = 'modules'}
    >
      Modules ({editedPlan.modules.length})
    </button>
    <button 
      class="tab" 
      class:active={selectedTab === 'days'}
      onclick={() => selectedTab = 'days'}
    >
      Days ({editedPlan.day_plans.length})
    </button>
    {#if editedPlan.validation_warnings.length > 0}
      <button 
        class="tab warning" 
        class:active={selectedTab === 'warnings'}
        onclick={() => selectedTab = 'warnings'}
      >
        Warnings ({editedPlan.validation_warnings.length})
      </button>
    {/if}
  </div>
  
  <div class="content">
    {#if selectedTab === 'overview'}
      <div class="section">
        <h2>Program Details</h2>
        <div class="form-group">
          <label for="title">Title</label>
          <input 
            id="title"
            type="text" 
            bind:value={editedPlan.program.title}
          />
        </div>
        <div class="form-group">
          <label for="description">Description</label>
          <textarea 
            id="description"
            bind:value={editedPlan.program.description}
            rows="4"
          ></textarea>
        </div>
        <div class="stats-grid">
          <div class="stat-card">
            <span class="stat-label">Total Days</span>
            <span class="stat-value">{editedPlan.day_plans.length}</span>
          </div>
          <div class="stat-card">
            <span class="stat-label">Modules</span>
            <span class="stat-value">{editedPlan.modules.length}</span>
          </div>
          <div class="stat-card">
            <span class="stat-label">Checklist Items</span>
            <span class="stat-value">{editedPlan.checklist_items.length}</span>
          </div>
          <div class="stat-card">
            <span class="stat-label">Quiz Questions</span>
            <span class="stat-value">{editedPlan.quiz_questions.length}</span>
          </div>
          <div class="stat-card">
            <span class="stat-label">Concept Tags</span>
            <span class="stat-value">{editedPlan.concept_tags.length}</span>
          </div>
          <div class="stat-card">
            <span class="stat-label">Dependencies</span>
            <span class="stat-value">{editedPlan.dependencies.length}</span>
          </div>
        </div>
      </div>
    {:else if selectedTab === 'modules'}
      <div class="section">
        <h2>Modules</h2>
        {#each editedPlan.modules as module, i}
          <div class="module-card">
            <div class="module-header" onclick={() => toggleModule(i)}>
              <div class="module-info">
                <div class="module-color" style="background: {module.color}"></div>
                <h3>{module.title}</h3>
                <span class="day-count">
                  {daysByModule()[i]?.length || 0} days
                </span>
              </div>
              <button class="expand-btn">
                {expandedModules.has(i) ? '−' : '+'}
              </button>
            </div>
            {#if expandedModules.has(i)}
              <div class="module-body">
                <div class="form-group">
                  <label>Title</label>
                  <input type="text" bind:value={module.title} />
                </div>
                <div class="form-group">
                  <label>Description</label>
                  <textarea bind:value={module.description} rows="3"></textarea>
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {:else if selectedTab === 'days'}
      <div class="section">
        <h2>Day Plans</h2>
        <div class="days-list">
          {#each editedPlan.day_plans as day}
            <div class="day-card">
              <div class="day-header">
                <span class="day-number">Day {day.day_number}</span>
                <span class="complexity">
                  {'⭐'.repeat(day.complexity_level)}
                </span>
              </div>
              <h4>{day.title}</h4>
              <div class="day-meta">
                <span>{day.estimated_minutes} min</span>
                <span>Module {day.module_index + 1}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>
    {:else if selectedTab === 'warnings'}
      <div class="section">
        <h2>Validation Warnings</h2>
        <div class="warnings-list">
          {#each editedPlan.validation_warnings as warning}
            <div class="warning-item">
              <span class="warning-icon">⚠️</span>
              <p>{warning}</p>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
  
  <div class="actions">
    <button class="save-btn" onclick={handleSave}>
      Save Changes
    </button>
  </div>
</div>

<style>
  .review-interface {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  .tabs {
    display: flex;
    gap: 0.5rem;
    padding: 1rem;
    background: #111827;
    border-bottom: 1px solid #374151;
  }
  
  .tab {
    padding: 0.75rem 1.5rem;
    background: transparent;
    border: none;
    color: #9ca3af;
    font-weight: 600;
    cursor: pointer;
    border-radius: 8px;
    transition: all 0.2s;
  }
  
  .tab:hover {
    background: #1f2937;
    color: #f3f4f6;
  }
  
  .tab.active {
    background: #6366f1;
    color: white;
  }
  
  .tab.warning {
    color: #fbbf24;
  }
  
  .content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }
  
  .section h2 {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f3f4f6;
    margin: 0 0 1.5rem 0;
  }
  
  .form-group {
    margin-bottom: 1.5rem;
  }
  
  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: #9ca3af;
    margin-bottom: 0.5rem;
  }
  
  .form-group input,
  .form-group textarea {
    width: 100%;
    padding: 0.75rem;
    background: #111827;
    border: 1px solid #374151;
    border-radius: 8px;
    color: #f3f4f6;
    font-family: inherit;
  }
  
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1rem;
    margin-top: 2rem;
  }
  
  .stat-card {
    padding: 1.5rem;
    background: #111827;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .stat-label {
    font-size: 0.875rem;
    color: #9ca3af;
  }
  
  .stat-value {
    font-size: 2rem;
    font-weight: 700;
    color: #f3f4f6;
  }
  
  .module-card {
    background: #111827;
    border-radius: 8px;
    margin-bottom: 1rem;
    overflow: hidden;
  }
  
  .module-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    cursor: pointer;
  }
  
  .module-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .module-color {
    width: 1rem;
    height: 1rem;
    border-radius: 4px;
  }
  
  .module-card h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0;
  }
  
  .day-count {
    font-size: 0.875rem;
    color: #9ca3af;
  }
  
  .expand-btn {
    background: none;
    border: none;
    color: #9ca3af;
    font-size: 1.5rem;
    cursor: pointer;
    width: 2rem;
    height: 2rem;
  }
  
  .module-body {
    padding: 0 1.5rem 1.5rem;
  }
  
  .days-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
  
  .day-card {
    padding: 1.5rem;
    background: #111827;
    border-radius: 8px;
    border: 1px solid #374151;
  }
  
  .day-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }
  
  .day-number {
    font-size: 0.875rem;
    font-weight: 600;
    color: #6366f1;
  }
  
  .complexity {
    font-size: 0.875rem;
  }
  
  .day-card h4 {
    font-size: 1rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0 0 0.75rem 0;
  }
  
  .day-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.875rem;
    color: #9ca3af;
  }
  
  .warnings-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .warning-item {
    display: flex;
    gap: 1rem;
    padding: 1rem;
    background: #7f1d1d;
    border-radius: 8px;
  }
  
  .warning-icon {
    font-size: 1.5rem;
  }
  
  .warning-item p {
    margin: 0;
    color: #fca5a5;
  }
  
  .actions {
    padding: 1.5rem;
    background: #111827;
    border-top: 1px solid #374151;
  }
  
  .save-btn {
    width: 100%;
    padding: 1rem;
    background: #6366f1;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .save-btn:hover {
    background: #4f46e5;
  }
</style>
