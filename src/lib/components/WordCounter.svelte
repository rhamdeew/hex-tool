<script lang="ts">
  export let words = 0;
  export let characters = 0;
</script>

<div class="word-counter">
  <span class="counter-item">
    <span class="counter-value">{words}</span>
    <span class="counter-label">words</span>
  </span>
  <span class="counter-divider">•</span>
  <span class="counter-item">
    <span class="counter-value">{characters}</span>
    <span class="counter-label">characters</span>
  </span>
</div>

<style>
  .word-counter {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    color: #666666;
  }

  :global(.dark .word-counter) {
    color: #a3a3a3;
  }

  .counter-item {
    display: flex;
    align-items: baseline;
    gap: 0.25rem;
  }

  .counter-value {
    font-weight: 600;
    color: #1a1a1a;
  }

  :global(.dark .counter-value) {
    color: #f5f5f5;
  }

  .counter-label {
    font-size: 0.75rem;
    text-transform: lowercase;
  }

  .counter-divider {
    color: #e5e5e5;
  }

  :global(.dark .counter-divider) {
    color: #404040;
  }
</style>
