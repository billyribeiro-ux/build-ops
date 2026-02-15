<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  
  interface Props {
    onFilesSelected: (paths: string[]) => void;
    accept?: string[];
    multiple?: boolean;
  }
  
  let { 
    onFilesSelected, 
    accept = ['pdf', 'md', 'markdown', 'txt'],
    multiple = true 
  }: Props = $props();
  
  let isDragging = $state(false);
  let selectedFiles = $state<string[]>([]);
  
  async function handleBrowse() {
    const selected = await open({
      multiple,
      filters: [{
        name: 'Documents',
        extensions: accept
      }]
    });
    
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected];
      selectedFiles = paths;
      onFilesSelected(paths);
    }
  }
  
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragging = true;
  }
  
  function handleDragLeave() {
    isDragging = false;
  }
  
  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragging = false;
    
    if (e.dataTransfer?.files) {
      const paths = Array.from(e.dataTransfer.files).map(f => f.name);
      selectedFiles = paths;
      onFilesSelected(paths);
    }
  }
  
  function removeFile(index: number) {
    selectedFiles = selectedFiles.filter((_, i) => i !== index);
    onFilesSelected(selectedFiles);
  }
</script>

<div 
  role="button"
  tabindex="0"
  class="upload-zone"
  class:dragging={isDragging}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
>
  <div class="upload-content">
    <svg class="upload-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
    </svg>
    
    <h3>Drop files here or click to browse</h3>
    <p>Supports PDF, Markdown, and Text files</p>
    
    <button type="button" class="browse-btn" onclick={handleBrowse}>
      Browse Files
    </button>
  </div>
  
  {#if selectedFiles.length > 0}
    <div class="selected-files">
      <h4>Selected Files ({selectedFiles.length})</h4>
      <ul>
        {#each selectedFiles as file, i (i)}
          <li>
            <span class="file-name">{file.split('/').pop()}</span>
            <button type="button" class="remove-btn" onclick={() => removeFile(i)}>
              ×
            </button>
          </li>
        {/each}
      </ul>
    </div>
  {/if}
</div>

<style>
  .upload-zone {
    border: 2px dashed #4b5563;
    border-radius: 12px;
    padding: 3rem;
    text-align: center;
    transition: all 0.2s;
    background: #1f2937;
  }
  
  .upload-zone.dragging {
    border-color: #6366f1;
    background: #312e81;
  }
  
  .upload-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
  
  .upload-icon {
    width: 4rem;
    height: 4rem;
    color: #9ca3af;
  }
  
  h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0;
  }
  
  p {
    color: #9ca3af;
    margin: 0;
  }
  
  .browse-btn {
    margin-top: 1rem;
    padding: 0.75rem 2rem;
    background: #6366f1;
    color: white;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .browse-btn:hover {
    background: #4f46e5;
  }
  
  .selected-files {
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 1px solid #374151;
  }
  
  .selected-files h4 {
    font-size: 1rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0 0 1rem 0;
    text-align: left;
  }
  
  .selected-files ul {
    list-style: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .selected-files li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    background: #111827;
    border-radius: 6px;
  }
  
  .file-name {
    color: #f3f4f6;
    font-size: 0.875rem;
  }
  
  .remove-btn {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 1.5rem;
    cursor: pointer;
    padding: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: background 0.2s;
  }
  
  .remove-btn:hover {
    background: #7f1d1d;
  }
</style>
