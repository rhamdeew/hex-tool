<script lang="ts">
  import { CheckCircle } from 'lucide-svelte';
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
            Editor: TipTap WYSIWYG
          </li>
          <li class="flex items-center gap-2">
            <CheckCircle class="w-4 h-4 text-success" />
            TypeScript Support
          </li>
        </ul>
      </div>

      <!-- Posts Page Card -->
      <div class="bg-surface dark:bg-dark-surface rounded-lg p-6 shadow-sm">
        <h2 class="text-2xl font-semibold text-text-primary dark:text-dark-text-primary mb-4">
          Posts
        </h2>

        <a
          href="/posts"
          class="inline-flex items-center gap-2 px-6 py-3 bg-accent hover:bg-blue-600 text-white rounded-lg font-medium transition-colors"
        >
          Manage Posts
        </a>
      </div>

      <!-- Editor Test Card -->
      <div class="bg-surface dark:bg-dark-surface rounded-lg p-6 shadow-sm">
        <h2 class="text-2xl font-semibold text-text-primary dark:text-dark-text-primary mb-4">
          Editor
        </h2>

        <a
          href="/editor"
          class="inline-flex items-center gap-2 px-6 py-3 bg-surface dark:bg-dark-surface hover:bg-gray-100 dark:hover:bg-gray-800 text-text-primary dark:text-dark-text-primary rounded-lg font-medium transition-colors"
        >
          Open Editor
        </a>
      </div>

      <div class="text-center text-text-secondary dark:text-dark-text-secondary text-sm">
        <p>✅ Phase 0 Complete: Project Setup</p>
        <p>✅ Week 1 Complete: Backend + File Operations</p>
        <p>✅ Week 2 Complete: TipTap Editor + UI Components</p>
        <p>✅ Week 3 Complete: Post List, Frontmatter Editor, Image Gallery</p>
        <p class="mt-2">Next: Full editor integration with backend</p>
      </div>
    </main>
  </div>
</div>
