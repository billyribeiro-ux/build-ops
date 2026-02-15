<script lang="ts">
  import { toasts } from '$lib/stores/app.svelte';
  import Toast from './Toast.svelte';
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
  {#each toasts.list as toast (toast.id)}
    <Toast
      message={toast.message}
      type={toast.type}
      duration={toast.duration}
      onclose={() => toasts.remove(toast.id)}
    />
  {/each}
</div>
