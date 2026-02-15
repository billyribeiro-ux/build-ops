<script lang="ts">
  import type { ImportJob } from '$lib/types';
  
  interface Props {
    job: ImportJob;
  }
  
  let { job }: Props = $props();
  
  const statusSteps = [
    { key: 'pending', label: 'Queued', icon: '⏳' },
    { key: 'extracting', label: 'Extracting Text', icon: '📄' },
    { key: 'analyzing', label: 'AI Analysis', icon: '🤖' },
    { key: 'generating', label: 'Generating Plan', icon: '⚙️' },
    { key: 'review', label: 'Ready for Review', icon: '✅' },
  ];
  
  const currentStepIndex = $derived(() => {
    const idx = statusSteps.findIndex(s => s.key === job.status);
    return idx >= 0 ? idx : 0;
  });
  
  const progress = $derived(() => {
    if (job.status === 'completed') return 100;
    if (job.status === 'failed') return 0;
    return ((currentStepIndex() + 1) / statusSteps.length) * 100;
  });
  
  const sourceFiles = $derived(() => {
    try {
      return JSON.parse(job.source_files_json);
    } catch {
      return [];
    }
  });
</script>

<div class="progress-card" class:failed={job.status === 'failed'}>
  <div class="header">
    <h3>Import Progress</h3>
    {#if job.status === 'failed'}
      <span class="status-badge error">Failed</span>
    {:else if job.status === 'completed'}
      <span class="status-badge success">Completed</span>
    {:else if job.status === 'review'}
      <span class="status-badge review">Ready for Review</span>
    {:else}
      <span class="status-badge processing">Processing...</span>
    {/if}
  </div>
  
  <div class="progress-bar">
    <div class="progress-fill" style="width: {progress()}%"></div>
  </div>
  
  <div class="steps">
    {#each statusSteps as step, i}
      <div 
        class="step" 
        class:active={i === currentStepIndex()}
        class:completed={i < currentStepIndex() || job.status === 'completed'}
      >
        <div class="step-icon">{step.icon}</div>
        <div class="step-label">{step.label}</div>
      </div>
    {/each}
  </div>
  
  <div class="stats">
    <div class="stat">
      <span class="stat-label">Files</span>
      <span class="stat-value">{sourceFiles.length}</span>
    </div>
    <div class="stat">
      <span class="stat-label">Pages</span>
      <span class="stat-value">{job.total_pages}</span>
    </div>
    <div class="stat">
      <span class="stat-label">Tokens</span>
      <span class="stat-value">{job.total_tokens.toLocaleString()}</span>
    </div>
    <div class="stat">
      <span class="stat-label">Days Generated</span>
      <span class="stat-value">{job.total_days_generated}</span>
    </div>
  </div>
  
  {#if job.error_message}
    <div class="error-message">
      <strong>Error in {job.error_step}:</strong>
      <p>{job.error_message}</p>
    </div>
  {/if}
</div>

<style>
  .progress-card {
    background: #1f2937;
    border-radius: 12px;
    padding: 2rem;
    border: 1px solid #374151;
  }
  
  .progress-card.failed {
    border-color: #ef4444;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0;
  }
  
  .status-badge {
    padding: 0.375rem 0.75rem;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 600;
  }
  
  .status-badge.processing {
    background: #1e3a8a;
    color: #93c5fd;
  }
  
  .status-badge.review {
    background: #065f46;
    color: #6ee7b7;
  }
  
  .status-badge.success {
    background: #065f46;
    color: #6ee7b7;
  }
  
  .status-badge.error {
    background: #7f1d1d;
    color: #fca5a5;
  }
  
  .progress-bar {
    height: 8px;
    background: #374151;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 2rem;
  }
  
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #6366f1, #8b5cf6);
    transition: width 0.3s ease;
  }
  
  .steps {
    display: flex;
    justify-content: space-between;
    margin-bottom: 2rem;
  }
  
  .step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    opacity: 0.4;
    transition: opacity 0.2s;
  }
  
  .step.active,
  .step.completed {
    opacity: 1;
  }
  
  .step-icon {
    font-size: 1.5rem;
  }
  
  .step-label {
    font-size: 0.75rem;
    color: #9ca3af;
    text-align: center;
  }
  
  .stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
    padding-top: 1.5rem;
    border-top: 1px solid #374151;
  }
  
  .stat {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .stat-label {
    font-size: 0.75rem;
    color: #9ca3af;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .stat-value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f3f4f6;
  }
  
  .error-message {
    margin-top: 1.5rem;
    padding: 1rem;
    background: #7f1d1d;
    border-radius: 8px;
    color: #fca5a5;
  }
  
  .error-message strong {
    display: block;
    margin-bottom: 0.5rem;
  }
  
  .error-message p {
    margin: 0;
    font-size: 0.875rem;
  }
</style>
