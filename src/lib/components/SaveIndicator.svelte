<script lang="ts">
  import { CheckCircle, Loader2, AlertCircle } from 'lucide-svelte';

  export let status: 'saved' | 'saving' | 'unsaved' | 'error' = 'saved';
  export let message = '';

  const statusConfig = {
    saved: {
      icon: CheckCircle,
      color: 'text-success',
      bgColor: 'bg-success/10',
      defaultMessage: 'Saved',
    },
    saving: {
      icon: Loader2,
      color: 'text-warning',
      bgColor: 'bg-warning/10',
      defaultMessage: 'Saving...',
    },
    unsaved: {
      icon: AlertCircle,
      color: 'text-error',
      bgColor: 'bg-error/10',
      defaultMessage: 'Unsaved changes',
    },
    error: {
      icon: AlertCircle,
      color: 'text-error',
      bgColor: 'bg-error/10',
      defaultMessage: 'Save failed',
    },
  };

  $: config = statusConfig[status];
</script>

<div class="save-indicator {config.bgColor}">
  <svelte:component this={config.icon} size={16} class="{config.color} spin={status === 'saving'}" />
  <span class="{config.color}">{message || config.defaultMessage}</span>
</div>

<style>
  .save-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
  }

  :global(.save-indicator svg.spin) {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  :global(.text-success) {
    color: #10b981;
  }

  :global(.bg-success\/10) {
    background-color: rgba(16, 185, 129, 0.1);
  }

  :global(.text-warning) {
    color: #f59e0b;
  }

  :global(.bg-warning\/10) {
    background-color: rgba(245, 158, 11, 0.1);
  }

  :global(.text-error) {
    color: #ef4444;
  }

  :global(.bg-error\/10) {
    background-color: rgba(239, 68, 68, 0.1);
  }

  /* Dark mode adjustments */
  :global(.dark .text-success) {
    color: #34d399;
  }

  :global(.dark .bg-success\/10) {
    background-color: rgba(52, 211, 153, 0.15);
  }

  :global(.dark .text-warning) {
    color: #fbbf24;
  }

  :global(.dark .bg-warning\/10) {
    background-color: rgba(251, 191, 36, 0.15);
  }

  :global(.dark .text-error) {
    color: #f87171;
  }

  :global(.dark .bg-error\/10) {
    background-color: rgba(248, 113, 113, 0.15);
  }
</style>
