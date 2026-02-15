<script lang="ts">
  import { goto } from '$app/navigation';
  import { imports } from '$lib/commands';
  import FileUploadZone from '$lib/components/import/FileUploadZone.svelte';
  import type { ImportJobSummary } from '$lib/types';
  import { onMount } from 'svelte';
  
  let selectedFiles = $state<string[]>([]);
  let apiKey = $state('');
  let isSubmitting = $state(false);
  let recentJobs = $state<ImportJobSummary[]>([]);
  let showApiKeyInput = $state(false);
  
  onMount(async () => {
    try {
      recentJobs = await imports.listImportJobs();
    } catch (e) {
      console.error('Failed to load recent jobs:', e);
    }
  });
  
  async function handleSubmit() {
    if (selectedFiles.length === 0) {
      alert('Please select at least one file');
      return;
    }
    
    if (!apiKey.trim()) {
      alert('Please enter your Anthropic API key');
      return;
    }
    
    isSubmitting = true;
    try {
      const job = await imports.startImport(selectedFiles, null, apiKey);
      goto(`/import/${job.id}`);
    } catch (e) {
      alert(`Failed to start import: ${e}`);
      isSubmitting = false;
    }
  }
  
  function formatDate(dateStr: string) {
    return new Date(dateStr).toLocaleDateString('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  }
  
  function getStatusColor(status: string) {
    switch (status) {
      case 'completed': return '#22c55e';
      case 'failed': return '#ef4444';
      case 'review': return '#f59e0b';
      default: return '#6366f1';
    }
  }
</script>

<div class="import-hub">
  <header>
    <h1>📚 Import Curriculum</h1>
    <p>Transform PDFs, Markdown, or Text files into structured learning programs</p>
  </header>
  
  <div class="main-content">
    <section class="upload-section">
      <h2>Upload Documents</h2>
      <FileUploadZone 
        onFilesSelected={(files) => selectedFiles = files}
      />
      
      <div class="api-key-section">
        <button 
          class="api-key-toggle"
          onclick={() => showApiKeyInput = !showApiKeyInput}
        >
          {showApiKeyInput ? '🔒 Hide' : '🔑 Enter'} API Key
        </button>
        
        {#if showApiKeyInput}
          <div class="api-key-input">
            <label for="api-key">Anthropic API Key</label>
            <input 
              id="api-key"
              type="password"
              bind:value={apiKey}
              placeholder="sk-ant-..."
            />
            <p class="help-text">
              Your API key is used only for this import and stored securely.
              <a href="https://console.anthropic.com/" target="_blank">Get API Key →</a>
            </p>
          </div>
        {/if}
      </div>
      
      <button 
        class="submit-btn"
        disabled={selectedFiles.length === 0 || !apiKey.trim() || isSubmitting}
        onclick={handleSubmit}
      >
        {isSubmitting ? 'Starting Import...' : 'Start Import'}
      </button>
    </section>
    
    {#if recentJobs.length > 0}
      <section class="recent-jobs">
        <h2>Recent Imports</h2>
        <div class="jobs-list">
          {#each recentJobs as job}
            <a href="/import/{job.id}" class="job-card">
              <div class="job-header">
                <span 
                  class="status-dot" 
                  style="background: {getStatusColor(job.status)}"
                ></span>
                <span class="job-type">{job.source_type}</span>
              </div>
              <div class="job-info">
                <span class="job-days">{job.total_days_generated} days</span>
                <span class="job-date">{formatDate(job.created_at)}</span>
              </div>
            </a>
          {/each}
        </div>
      </section>
    {/if}
  </div>
</div>

<style>
  .import-hub {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  header {
    text-align: center;
    margin-bottom: 3rem;
  }
  
  h1 {
    font-size: 2.5rem;
    font-weight: 800;
    color: #f3f4f6;
    margin: 0 0 0.5rem 0;
  }
  
  header p {
    font-size: 1.125rem;
    color: #9ca3af;
    margin: 0;
  }
  
  .main-content {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2rem;
  }
  
  @media (max-width: 768px) {
    .main-content {
      grid-template-columns: 1fr;
    }
  }
  
  section {
    background: #1f2937;
    border-radius: 12px;
    padding: 2rem;
    border: 1px solid #374151;
  }
  
  h2 {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f3f4f6;
    margin: 0 0 1.5rem 0;
  }
  
  .api-key-section {
    margin: 2rem 0;
  }
  
  .api-key-toggle {
    width: 100%;
    padding: 1rem;
    background: #111827;
    border: 1px solid #374151;
    border-radius: 8px;
    color: #f3f4f6;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .api-key-toggle:hover {
    background: #1f2937;
    border-color: #6366f1;
  }
  
  .api-key-input {
    margin-top: 1rem;
    padding: 1.5rem;
    background: #111827;
    border-radius: 8px;
  }
  
  .api-key-input label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: #9ca3af;
    margin-bottom: 0.5rem;
  }
  
  .api-key-input input {
    width: 100%;
    padding: 0.75rem;
    background: #1f2937;
    border: 1px solid #374151;
    border-radius: 6px;
    color: #f3f4f6;
    font-family: 'Monaco', monospace;
    font-size: 0.875rem;
  }
  
  .help-text {
    margin-top: 0.75rem;
    font-size: 0.875rem;
    color: #9ca3af;
  }
  
  .help-text a {
    color: #6366f1;
    text-decoration: none;
  }
  
  .help-text a:hover {
    text-decoration: underline;
  }
  
  .submit-btn {
    width: 100%;
    padding: 1.25rem;
    background: #6366f1;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 700;
    font-size: 1.125rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .submit-btn:hover:not(:disabled) {
    background: #4f46e5;
    transform: translateY(-2px);
  }
  
  .submit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .jobs-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .job-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: #111827;
    border-radius: 8px;
    border: 1px solid #374151;
    text-decoration: none;
    transition: all 0.2s;
  }
  
  .job-card:hover {
    border-color: #6366f1;
    transform: translateX(4px);
  }
  
  .job-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  
  .status-dot {
    width: 0.75rem;
    height: 0.75rem;
    border-radius: 50%;
  }
  
  .job-type {
    font-size: 0.875rem;
    font-weight: 600;
    color: #f3f4f6;
    text-transform: uppercase;
  }
  
  .job-info {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
  }
  
  .job-days {
    font-size: 1rem;
    font-weight: 700;
    color: #6366f1;
  }
  
  .job-date {
    font-size: 0.75rem;
    color: #9ca3af;
  }
</style>
