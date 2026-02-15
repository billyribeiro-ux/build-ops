<script lang="ts">
  import Icon from '@iconify/svelte';

  let {
    message,
    type = 'info',
    duration = 3000,
    onclose
  }: {
    message: string;
    type?: 'success' | 'error' | 'warning' | 'info';
    duration?: number;
    onclose?: () => void;
  } = $props();

  const icons = {
    success: 'ph:check-circle-bold',
    error: 'ph:warning-circle-bold',
    warning: 'ph:warning-bold',
    info: 'ph:info-bold'
  };

  const colors = {
    success: 'bg-accent-success/10 border-accent-success/30 text-accent-success',
    error: 'bg-accent-danger/10 border-accent-danger/30 text-accent-danger',
    warning: 'bg-accent-warning/10 border-accent-warning/30 text-accent-warning',
    info: 'bg-accent-info/10 border-accent-info/30 text-accent-info'
  };

  $effect(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        onclose?.();
      }, duration);
      return () => clearTimeout(timer);
    }
  });
</script>

<div
  class="flex items-center gap-3 rounded-lg border px-4 py-3 shadow-lg {colors[type]}"
  role="alert"
>
  <Icon icon={icons[type]} width="20" />
  <p class="flex-1 text-sm font-medium">{message}</p>
  <button
    onclick={onclose}
    class="rounded p-1 transition-colors hover:bg-black/10"
    aria-label="Close"
  >
    <Icon icon="ph:x-bold" width="16" />
  </button>
</div>
