<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { imports } from '$lib/commands';
  import ImportReviewInterface from '$lib/components/import/ImportReviewInterface.svelte';
  import type { ImportGeneratedPlan } from '$lib/types';
  import { onMount } from 'svelte';
  
  const jobId = $derived($page.params.id);
  let plan = $state<ImportGeneratedPlan | null>(null);
  let error = $state<string | null>(null);
  let isApplying = $state(false);
  
  onMount(async () => {
    try {
      plan = await imports.getImportPreview(jobId);
    } catch (e) {
      error = `Failed to load import preview: ${e}`;
    }
  });
  
  async function handleSave(updatedPlan: ImportGeneratedPlan) {
    try {
      const planJson = JSON.stringify(updatedPlan);
      await imports.updateImportPreview(jobId, planJson);
      alert('Changes saved successfully!');
    } catch (e) {
      alert(`Failed to save changes: ${e}`);
    }
  }
  
  async function handleApply() {
    if (!confirm('Apply this import? This will create the program in your database.')) {
      return;
    }
    
    isApplying = true;
    try {
      const program = await imports.applyImport(jobId);
      goto(`/programs/${program.id}`);
    } catch (e) {
      alert(`Failed to apply import: ${e}`);
      isApplying = false;
    }
  }
</script>

<div class="review-page">
  <header>
    <div class="header-content">
      <div>
        <a href="/import/{jobId}" class="back-link">← Back to Progress</a>
        <h1>Review Import</h1>
        <p>Review and edit the generated curriculum before applying</p>
      </div>
      <button 
        class="apply-btn"
        disabled={isApplying || !plan}
        onclick={handleApply}
      >
        {isApplying ? 'Applying...' : '✓ Apply Import'}
      </button>
    </div>
  </header>
  
  {#if error}
    <div class="error-banner">
      <p>{error}</p>
    </div>
  {:else if plan}
    <div class="review-container">
      <ImportReviewInterface {plan} onSave={handleSave} />
    </div>
  {:else}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading preview...</p>
    </div>
  {/if}
</div>

<style>
  .review-page {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: #0f172a;
  }
  
  header {
    background: #1f2937;
    border-bottom: 1px solid #374151;
    padding: 1.5rem 2rem;
  }
  
  .header-content {
    max-width: 1400px;
    margin: 0 auto;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .back-link {
    display: inline-block;
    color: #6366f1;
    text-decoration: none;
    font-weight: 600;
    margin-bottom: 0.5rem;
  }
  
  .back-link:hover {
    text-decoration: underline;
  }
  
  h1 {
    font-size: 1.75rem;
    font-weight: 800;
    color: #f3f4f6;
    margin: 0 0 0.25rem 0;
  }
  
  header p {
    color: #9ca3af;
    margin: 0;
  }
  
  .apply-btn {
    padding: 1rem 2rem;
    background: #22c55e;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 700;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .apply-btn:hover:not(:disabled) {
    background: #16a34a;
    transform: translateY(-2px);
  }
  
  .apply-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .error-banner {
    max-width: 1400px;
    margin: 2rem auto;
    padding: 1.5rem;
    background: #7f1d1d;
    border-radius: 12px;
    color: #fca5a5;
  }
  
  .error-banner p {
    margin: 0;
  }
  
  .review-container {
    flex: 1;
    overflow: hidden;
    max-width: 1400px;
    width: 100%;
    margin: 0 auto;
  }
  
  .loading {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
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
