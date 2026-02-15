<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import TopBar from '$lib/components/layout/TopBar.svelte';
  import ToastContainer from '$lib/components/ui/ToastContainer.svelte';

  let { children } = $props();

  let sidebarCollapsed = $state(false);
  let sidebarWidth = $derived(sidebarCollapsed ? 48 : 240);
</script>

<div class="flex h-screen bg-bg-primary">
  <Sidebar collapsed={sidebarCollapsed} />

  <div class="flex flex-1 flex-col" style="margin-left: {sidebarWidth}px;">
    <TopBar {sidebarWidth} />

    <main class="flex-1 overflow-y-auto pt-12">
      <div class="mx-auto max-w-7xl p-6">
        {@render children()}
      </div>
    </main>
  </div>
</div>

<ToastContainer />
<CommandPalette bind:isOpen={showCommandPalette} />
