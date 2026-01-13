<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, FolderOpen } from 'lucide-svelte';
  import { backend } from '$lib/services/backend';
  import { PostList } from '$lib/components';
  import { ImageGallery } from '$lib/components';
  import type { Post, ImageInfo } from '$lib/types';

  let posts: Post[] = $state([]);
  let images: ImageInfo[] = $state([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let showImageGallery = $state(false);
  let pendingImageField:
    | { field: 'listImage' | 'mainImage'; post: Post }
    | null = null;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    error = null;

    try {
      // Check if project is selected
      const projectPath = backend.getProjectPath();
      if (!projectPath) {
        error = 'No project selected. Please select a project folder first.';
        return;
      }

      const [postsData, imagesData] = await Promise.all([
        backend.listPosts(),
        backend.listImages(),
      ]);

      posts = postsData;
      images = imagesData;
    } catch (err) {
      console.error('Failed to load data:', err);
      error = err instanceof Error ? err.message : 'Failed to load data';
    } finally {
      loading = false;
    }
  }

  async function handleCreatePost() {
    try {
      const title = prompt('Enter post title:');
      if (!title) return;

      const newPost = await backend.createPost(title);
      // Navigate to editor
      window.location.href = `/editor?id=${encodeURIComponent(newPost.id)}`;
    } catch (err) {
      console.error('Failed to create post:', err);
      alert('Failed to create post: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  }

  function handleEdit(post: Post) {
    window.location.href = `/editor?id=${encodeURIComponent(post.id)}`;
  }

  async function handleDelete(post: Post) {
    try {
      await backend.deletePost(post.id);
      // Reload posts
      posts = await backend.listPosts();
    } catch (err) {
      console.error('Failed to delete post:', err);
      alert('Failed to delete post: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  }

  async function handleSelectProject() {
    try {
      await backend.selectProjectFolder();
      await loadData();
    } catch (err) {
      console.error('Failed to select project:', err);
      alert('Failed to select project: ' + (err instanceof Error ? err.message : 'Unknown error'));
    }
  }

  function handleImageSelect(image: ImageInfo) {
    // This will be called when an image is selected from gallery
    if (pendingImageField) {
      const { field, post } = pendingImageField;
      // Update the post's frontmatter
      if (field === 'listImage') {
        post.frontmatter.listImage = image.url;
      } else if (field === 'mainImage') {
        post.frontmatter.mainImage = image.url;
      }
      pendingImageField = null;
    }
  }

  function handleImageDelete(image: ImageInfo) {
    // Delete image and reload
    backend.deleteImage(image.path).then(() => {
      images = images.filter((img) => img.fullPath !== image.fullPath);
    });
  }

  function handleUploadImage() {
    // Trigger file upload dialog
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';

    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;

      try {
        // In Tauri, file.path should be available
        // @ts-ignore - Tauri-specific property
        const sourcePath = file.path || file.name;
        const imageUrl = await backend.copyImageToProject(sourcePath);

        // Reload images
        images = await backend.listImages();
      } catch (err) {
        console.error('Failed to upload image:', err);
        alert('Failed to upload image: ' + (err instanceof Error ? err.message : 'Unknown error'));
      }
    };

    input.click();
  }

  function openImageGalleryForPost(field: 'listImage' | 'mainImage', post: Post) {
    pendingImageField = { field, post };
    showImageGallery = true;
  }
</script>

<div class="posts-page">
  <!-- Header -->
  <header class="page-header">
    <div class="header-left">
      <h1 class="page-title">Posts</h1>
      <p class="page-subtitle">Manage your Hexo blog posts</p>
    </div>

    <div class="header-right">
      {#if !backend.getProjectPath()}
        <button class="select-project-btn" onclick={handleSelectProject} type="button">
          <FolderOpen size={18} />
          <span>Select Project</span>
        </button>
      {:else}
        <button class="create-btn" onclick={handleCreatePost} type="button">
          <Plus size={18} />
          <span>New Post</span>
        </button>
      {/if}
    </div>
  </header>

  <!-- Content -->
  <main class="page-content">
    {#if loading}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading posts...</p>
      </div>
    {:else if error}
      <div class="error-state">
        <div class="error-icon">⚠️</div>
        <h3>Error</h3>
        <p>{error}</p>
        <button class="retry-btn" onclick={handleSelectProject} type="button">
          <FolderOpen size={18} />
          Select Project Folder
        </button>
      </div>
    {:else if posts.length === 0}
      <div class="empty-state">
        <div class="empty-icon">📝</div>
        <h3>No posts yet</h3>
        <p>Create your first post to get started</p>
        <button class="create-btn-empty" onclick={handleCreatePost} type="button">
          <Plus size={18} />
          Create Post
        </button>
      </div>
    {:else}
      <PostList
        {posts}
        onCreate={handleCreatePost}
        onEdit={handleEdit}
        onDelete={handleDelete}
      />
    {/if}
  </main>

  <!-- Image Gallery Modal -->
  <ImageGallery
    bind:open={showImageGallery}
    {images}
    onSelect={handleImageSelect}
    onDelete={handleImageDelete}
    onUpload={handleUploadImage}
  />
</div>

<style>
  .posts-page {
    min-height: 100vh;
    background-color: #ffffff;
    display: flex;
    flex-direction: column;
  }

  :global(.dark .posts-page) {
    background-color: #1a1a1a;
  }

  /* Header */
  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem 2rem;
    border-bottom: 1px solid #e5e5e5;
    background-color: #ffffff;
  }

  :global(.dark .page-header) {
    background-color: #2d2d2d;
    border-bottom-color: #404040;
  }

  .page-title {
    font-size: 1.875rem;
    font-weight: 700;
    color: #1a1a1a;
    margin: 0;
  }

  :global(.dark .page-title) {
    color: #f5f5f5;
  }

  .page-subtitle {
    font-size: 0.875rem;
    color: #666666;
    margin: 0.25rem 0 0 0;
  }

  :global(.dark .page-subtitle) {
    color: #a3a3a3;
  }

  .header-right {
    display: flex;
    gap: 0.75rem;
  }

  .create-btn,
  .select-project-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .create-btn:hover,
  .select-project-btn:hover {
    background-color: #2563eb;
  }

  .create-btn-empty {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .create-btn-empty:hover {
    background-color: #2563eb;
  }

  /* Content */
  .page-content {
    flex: 1;
    padding: 2rem;
    overflow-y: auto;
  }

  /* Loading State */
  .loading-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    gap: 1rem;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid #e5e5e5;
    border-top-color: #3b82f6;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .loading-state p {
    font-size: 0.875rem;
    color: #666666;
    margin: 0;
  }

  :global(.dark .loading-state p) {
    color: #a3a3a3;
  }

  /* Error State */
  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  .error-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .error-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #1a1a1a;
    margin: 0 0 0.5rem 0;
  }

  :global(.dark .error-state h3) {
    color: #f5f5f5;
  }

  .error-state p {
    font-size: 0.875rem;
    color: #666666;
    margin: 0 0 1rem 0;
  }

  :global(.dark .error-state p) {
    color: #a3a3a3;
  }

  .retry-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1.25rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .retry-btn:hover {
    background-color: #2563eb;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #1a1a1a;
    margin: 0 0 0.5rem 0;
  }

  :global(.dark .empty-state h3) {
    color: #f5f5f5;
  }

  .empty-state p {
    font-size: 0.875rem;
    color: #666666;
    margin: 0;
  }

  :global(.dark .empty-state p) {
    color: #a3a3a3;
  }
</style>
