<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { imports } from '$lib/commands';
  import ImportProgressCard from '$lib/components/import/ImportProgressCard.svelte';
  import type { ImportJob } from '$lib/types';
  import { onMount, onDestroy } from 'svelte';
  
  const jobId = $derived($page.params.id);
  let job = $state<ImportJob | null>(null);
  let error = $state<string | null>(null);
  let pollInterval: number | null = null;
  
  async function loadJob() {
    try {
      job = await imports.getImportJob(jobId);
      
      if (job.status === 'review') {
        goto(`/import/${jobId}/review`);
      } else if (job.status === 'completed') {
        clearInterval(pollInterval!);
      } else if (job.status === 'failed') {
        clearInterval(pollInterval!);
      }
    } catch (e) {
      error = `Failed to load import job: ${e}`;
      clearInterval(pollInterval!);
    }
  }
  
  onMount(() => {
    loadJob();
    pollInterval = setInterval(loadJob, 2000) as unknown as number;
  });
  
  onDestroy(() => {
    if (pollInterval) {
      clearInterval(pollInterval);
    }
  });
  
  async function handleCancel() {
    if (!confirm('Are you sure you want to cancel this import?')) return;
    
    try {
      await imports.cancelImport(jobId);
      goto('/import');
    } catch (e) {
      alert(`Failed to cancel import: ${e}`);
    }
  }
  
  async function handleRetry() {
    const apiKey = prompt('Enter your Anthropic API key to retry:');
    if (!apiKey) return;
    
    try {
      await imports.retryImport(jobId, apiKey);
      loadJob();
      pollInterval = setInterval(loadJob, 2000) as unknown as number;
    } catch (e) {
      alert(`Failed to retry import: ${e}`);
    }
  }
</script>

<div class="import-progress-page">
  <header>
    <a href="/import" class="back-link">← Back to Imports</a>
    <h1>Import Progress</h1>
  </header>
  
  {#if error}
    <div class="error-banner">
      <p>{error}</p>
    </div>
  {:else if job}
    <div class="content">
      <ImportProgressCard {job} />
      
      <div class="actions">
        {#if job.status === 'failed'}
          <button class="retry-btn" onclick={handleRetry}>
            🔄 Retry Import
          </button>
        {:else if job.status !== 'completed' && job.status !== 'review'}
          <button class="cancel-btn" onclick={handleCancel}>
            ✕ Cancel Import
          </button>
        {/if}
        
        {#if job.status === 'review'}
          <a href="/import/{jobId}/review" class="review-btn">
            Review & Apply →
          </a>
        {/if}
        
        {#if job.status === 'completed' && job.program_id}
          <a href="/programs/{job.program_id}" class="view-program-btn">
            View Program →
          </a>
        {/if}
      </div>
    </div>
  {:else}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading import job...</p>
    </div>
  {/if}
</div>

<style>
  .import-progress-page {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  header {
    margin-bottom: 2rem;
  }
  
  .back-link {
    display: inline-block;
    color: #6366f1;
    text-decoration: none;
    font-weight: 600;
    margin-bottom: 1rem;
  }
  
  .back-link:hover {
    text-decoration: underline;
  }
  
  h1 {
    font-size: 2rem;
    font-weight: 800;
    color: #f3f4f6;
    margin: 0;
  }
  
  .error-banner {
    padding: 1.5rem;
    background: #7f1d1d;
    border-radius: 12px;
    color: #fca5a5;
  }
  
  .error-banner p {
    margin: 0;
  }
  
  .content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .actions {
    display: flex;
    gap: 1rem;
  }
  
  .actions button,
  .actions a {
    flex: 1;
    padding: 1rem;
    border: none;
    border-radius: 8px;
    font-weight: 700;
    font-size: 1rem;
    cursor: pointer;
    text-decoration: none;
    text-align: center;
    transition: all 0.2s;
  }
  
  .retry-btn {
    background: #f59e0b;
    color: white;
  }
  
  .retry-btn:hover {
    background: #d97706;
  }
  
  .cancel-btn {
    background: #ef4444;
    color: white;
  }
  
  .cancel-btn:hover {
    background: #dc2626;
  }
  
  .review-btn {
    background: #6366f1;
    color: white;
  }
  
  .review-btn:hover {
    background: #4f46e5;
  }
  
  .view-program-btn {
    background: #22c55e;
    color: white;
  }
  
  .view-program-btn:hover {
    background: #16a34a;
  }
  
  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    gap: 1rem;
  }
  
  .spinner {
    width: 3rem;
    height: 3rem;
    border: 4px solid #374151;
    border-top-color: #6366f1;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .loading p {
    color: #9ca3af;
    font-size: 1.125rem;
  }
</style>
