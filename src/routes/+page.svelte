<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { CheckCircle, XCircle, Loader2 } from 'lucide-svelte';

  let connectionStatus = $state<'idle' | 'loading' | 'success' | 'error'>('idle');
  let connectionMessage = $state('');

  async function testConnection() {
    connectionStatus = 'loading';
    connectionMessage = '';

    try {
      const result = await invoke<string>('test_connection');
      connectionStatus = 'success';
      connectionMessage = result;
    } catch (error) {
      connectionStatus = 'error';
      connectionMessage = error as string;
    }
  }

  async function greet() {
    try {
      const result = await invoke<string>('greet', { name: 'Hexo Editor' });
      alert(result);
    } catch (error) {
      alert('Error: ' + error);
    }
  }
</script>

<div class="min-h-screen bg-background dark:bg-dark-background p-8">
  <div class="max-w-4xl mx-auto">
    <header class="text-center mb-12">
      <h1 class="text-4xl font-bold text-text-primary dark:text-dark-text-primary mb-4">
        Hex Tool
      </h1>
      <p class="text-text-secondary dark:text-dark-text-secondary">
        Hexo Blog Editor - Built with Tauri + Svelte
      </p>
    </header>

    <main class="space-y-8">
      <!-- Connection Test Card -->
      <div class="bg-surface dark:bg-dark-surface rounded-lg p-6 shadow-sm">
        <h2 class="text-2xl font-semibold text-text-primary dark:text-dark-text-primary mb-4">
          Backend Connection Test
        </h2>

        <div class="space-y-4">
          <button
            onclick={testConnection}
            disabled={connectionStatus === 'loading'}
            class="px-6 py-3 bg-accent hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {#if connectionStatus === 'loading'}
              <Loader2 class="w-5 h-5 animate-spin" />
              Testing...
            {:else}
              Test Connection
            {/if}
          </button>

          {#if connectionStatus === 'success'}
            <div class="flex items-center gap-2 text-success dark:text-dark-success">
              <CheckCircle class="w-5 h-5" />
              <span class="font-medium">{connectionMessage}</span>
            </div>
          {:else if connectionStatus === 'error'}
            <div class="flex items-center gap-2 text-error dark:text-dark-error">
              <XCircle class="w-5 h-5" />
              <span class="font-medium">{connectionMessage}</span>
            </div>
          {/if}
        </div>
      </div>

      <!-- Greet Test Card -->
      <div class="bg-surface dark:bg-dark-surface rounded-lg p-6 shadow-sm">
        <h2 class="text-2xl font-semibold text-text-primary dark:text-dark-text-primary mb-4">
          Greet Command Test
        </h2>

        <button
          onclick={greet}
          class="px-6 py-3 bg-accent hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
        >
          Call Greet Command
        </button>
      </div>

      <!-- Project Info Card -->
      <div class="bg-surface dark:bg-dark-surface rounded-lg p-6 shadow-sm">
        <h2 class="text-2xl font-semibold text-text-primary dark:text-dark-text-primary mb-4">
          Project Status
        </h2>

        <ul class="space-y-2 text-text-secondary dark:text-dark-text-secondary">
          <li class="flex items-center gap-2">
            <CheckCircle class="w-4 h-4 text-success" />
            Frontend: Svelte + SvelteKit + TailwindCSS
          </li>
          <li class="flex items-center gap-2">
            <CheckCircle class="w-4 h-4 text-success" />
            Backend: Rust + Tauri 2
          </li>
          <li class="flex items-center gap-2">
            <CheckCircle class="w-4 h-4 text-success" />
            Icons: Lucide Svelte
          </li>
          <li class="flex items-center gap-2">
            <CheckCircle class="w-4 h-4 text-success" />
            TypeScript Support
          </li>
        </ul>
      </div>

      <div class="text-center text-text-secondary dark:text-dark-text-secondary text-sm">
        <p>Phase 0 Complete: Project Setup</p>
        <p class="mt-2">Next: Implement MVP features</p>
      </div>
    </main>
  </div>
</div>
