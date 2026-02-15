<script lang="ts">
  let {
    children,
    variant = 'default',
    size = 'medium'
  }: {
    children: any;
    variant?: 'default' | 'success' | 'warning' | 'danger' | 'info' | 'purple';
    size?: 'small' | 'medium' | 'large';
  } = $props();

  const variantClasses = {
    default: 'bg-bg-hover text-text-secondary',
    success: 'bg-accent-success/10 text-accent-success',
    warning: 'bg-accent-warning/10 text-accent-warning',
    danger: 'bg-accent-danger/10 text-accent-danger',
    info: 'bg-accent-info/10 text-accent-info',
    purple: 'bg-accent-mastery/10 text-accent-mastery'
  };

  const sizeClasses = {
    small: 'px-2 py-0.5 text-xs',
    medium: 'px-2.5 py-1 text-sm',
    large: 'px-3 py-1.5 text-base'
  };
</script>

<span
  class="inline-flex items-center gap-1.5 rounded-full font-medium {variantClasses[variant]} {sizeClasses[size]}"
>
  {@render children()}
</span>
