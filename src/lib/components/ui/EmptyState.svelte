<script lang="ts">
  import Icon from '@iconify/svelte';

  let {
    icon,
    title,
    description,
    action
  }: {
    icon: string;
    title: string;
    description: string;
    action?: any;
  } = $props();
</script>

<div class="flex flex-col items-center justify-center rounded-xl border border-border-primary bg-bg-secondary p-12 text-center">
  <Icon {icon} width="48" class="mb-4 text-text-tertiary" />
  <h3 class="mb-2 text-lg font-semibold text-text-primary">{title}</h3>
  <p class="mb-6 max-w-md text-text-secondary">{description}</p>
  {#if action}
    {@render action()}
  {/if}
</div>
