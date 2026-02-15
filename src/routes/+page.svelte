<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<div class="space-y-6">
  <div class="flex items-center gap-3">
    <Icon icon="ph:house-bold" width="28" class="text-accent-primary" />
    <h1 class="text-2xl font-bold text-text-primary">Dashboard</h1>
  </div>

  <div class="rounded-xl border border-border-primary bg-bg-secondary p-8 text-center">
    <Icon icon="ph:rocket-launch-bold" width="48" class="mx-auto mb-4 text-accent-primary" />
    <h2 class="mb-2 text-xl font-semibold text-text-primary">Welcome to BuildOps 40</h2>
    <p class="text-text-secondary">
      Your engineering learning OS is ready. Create your first program to get started.
    </p>
    <a
      href="/programs/new"
      class="mt-6 inline-flex items-center gap-2 rounded-lg bg-accent-primary px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-accent-primary/90"
    >
      <Icon icon="ph:plus-bold" width="16" />
      Create Program
    </a>
  </div>
</div>
