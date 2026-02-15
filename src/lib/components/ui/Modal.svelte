<script lang="ts">
  import Icon from '@iconify/svelte';

  let {
    open = $bindable(false),
    title,
    children,
    size = 'medium',
    onclose
  }: {
    open?: boolean;
    title: string;
    children: any;
    size?: 'small' | 'medium' | 'large' | 'full';
    onclose?: () => void;
  } = $props();

  const sizeClasses = {
    small: 'max-w-md',
    medium: 'max-w-2xl',
    large: 'max-w-4xl',
    full: 'max-w-7xl'
  };

  function handleClose() {
    open = false;
    onclose?.();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      handleClose();
    }
  }

  $effect(() => {
    if (open) {
      document.body.style.overflow = 'hidden';
      return () => {
        document.body.style.overflow = '';
      };
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm"
    onclick={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="modal-title"
  >
    <div
      class="relative w-full {sizeClasses[size]} mx-4 rounded-xl border border-border-primary bg-bg-secondary shadow-2xl"
    >
      <div class="flex items-center justify-between border-b border-border-primary px-6 py-4">
        <h2 id="modal-title" class="text-lg font-semibold text-text-primary">{title}</h2>
        <button
          onclick={handleClose}
          class="rounded-lg p-1 text-text-tertiary transition-colors hover:bg-bg-hover hover:text-text-primary"
          aria-label="Close modal"
        >
          <Icon icon="ph:x-bold" width="20" />
        </button>
      </div>
      <div class="max-h-[calc(100vh-12rem)] overflow-y-auto px-6 py-4">
        {@render children()}
      </div>
    </div>
  </div>
{/if}
