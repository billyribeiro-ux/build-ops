<script lang="ts">
  import Icon from '@iconify/svelte';
  import { page } from '$app/stores';
  import { mainNavItems, bottomNavItems } from '$lib/config/navigation';

  let { collapsed = false }: { collapsed?: boolean } = $props();

  let currentPath = $derived($page.url.pathname);

  function isActive(href: string): boolean {
    if (href === '/') return currentPath === '/';
    return currentPath.startsWith(href);
  }
</script>

<aside
  class="fixed left-0 top-0 bottom-0 z-40 flex flex-col border-r border-border-primary bg-bg-secondary transition-all duration-200"
  style="width: {collapsed ? '48px' : '240px'}; padding-top: 48px;"
>
  <!-- Logo -->
  <div class="flex items-center gap-3 px-4 py-4">
    {#if !collapsed}
      <span class="text-lg font-bold text-text-primary">BuildOps 40</span>
    {/if}
  </div>

  <!-- Main nav -->
  <nav class="flex flex-1 flex-col gap-1 px-2">
    {#each mainNavItems as item (item.href)}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {isActive(item.href) ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <Icon icon={item.icon} width="20" />
        {#if !collapsed}
          <span>{item.label}</span>
          {#if item.shortcut}
            <span class="ml-auto text-xs text-text-tertiary">{item.shortcut}</span>
          {/if}
        {/if}
      </a>
    {/each}
  </nav>

  <!-- Bottom nav -->
  <nav class="flex flex-col gap-1 px-2 pb-4">
    {#each bottomNavItems as item (item.href)}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {isActive(item.href) ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <Icon icon={item.icon} width="20" />
        {#if !collapsed}
          <span>{item.label}</span>
        {/if}
      </a>
    {/each}
  </nav>
</aside>
