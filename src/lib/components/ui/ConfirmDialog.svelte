<script lang="ts">
  import Modal from './Modal.svelte';
  import Button from './Button.svelte';

  let {
    open = $bindable(false),
    title,
    message,
    confirmText = 'Confirm',
    cancelText = 'Cancel',
    variant = 'danger',
    onconfirm,
    oncancel
  }: {
    open?: boolean;
    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    variant?: 'primary' | 'secondary' | 'outline' | 'ghost' | 'danger' | 'success';
    onconfirm: () => void;
    oncancel?: () => void;
  } = $props();

  function handleConfirm() {
    onconfirm();
    open = false;
  }

  function handleCancel() {
    oncancel?.();
    open = false;
  }
</script>

<Modal bind:open {title} size="small" onclose={handleCancel}>
  <div class="space-y-4">
    <p class="text-text-secondary">{message}</p>
    <div class="flex gap-3">
      <Button variant="outline" onclick={handleCancel} class="flex-1">
        {cancelText}
      </Button>
      <Button {variant} onclick={handleConfirm} class="flex-1">
        {confirmText}
      </Button>
    </div>
  </div>
</Modal>
