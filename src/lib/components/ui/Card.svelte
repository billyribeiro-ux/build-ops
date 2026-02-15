<script lang="ts">
  let {
    children,
    class: className = '',
    padding = 'normal',
    hover = false,
    onclick
  }: {
    children: any;
    class?: string;
    padding?: 'none' | 'small' | 'normal' | 'large';
    hover?: boolean;
    onclick?: () => void;
  } = $props();

  const paddingClasses = {
    none: '',
    small: 'p-4',
    normal: 'p-6',
    large: 'p-8'
  };

  const hoverClass = $derived(hover ? 'transition-colors hover:border-border-secondary cursor-pointer' : '');
</script>

<div
  class="rounded-xl border border-border-primary bg-bg-secondary {paddingClasses[padding]} {hoverClass} {className}"
  onclick={onclick}
  onkeydown={(e) => onclick && (e.key === 'Enter' || e.key === ' ') && onclick()}
  role={onclick ? 'button' : undefined}
  tabindex={onclick ? 0 : undefined}
>
  {@render children()}
</div>
