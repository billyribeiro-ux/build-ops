<script lang="ts">
  import Icon from '@iconify/svelte';

  let { sidebarWidth = 240 }: { sidebarWidth?: number } = $props();
</script>

<header
  class="titlebar-drag-region fixed top-0 right-0 z-50 flex h-12 items-center border-b border-border-primary bg-bg-secondary/80 backdrop-blur-md"
  style="left: {sidebarWidth}px;"
>
  <!-- macOS traffic lights occupy left ~70px, this bar starts after sidebar -->
  <div class="titlebar-no-drag flex flex-1 items-center gap-4 px-4">
    <!-- Search trigger -->
    <button
      class="flex items-center gap-2 rounded-lg border border-border-primary bg-bg-tertiary px-3 py-1.5 text-sm text-text-tertiary transition-colors hover:border-border-secondary hover:text-text-secondary"
      onclick={() => {/* Command palette - implemented in Phase 10 */}}
    >
      <Icon icon="ph:magnifying-glass" width="16" />
      <span>Search...</span>
      <kbd class="ml-4 rounded bg-bg-hover px-1.5 py-0.5 text-xs">⌘K</kbd>
    </button>
  </div>
</header>
