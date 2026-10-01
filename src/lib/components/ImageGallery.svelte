<script lang="ts">
  import {
    X,
    Search,
    Upload as UploadIcon,
    Trash2,
    Image as ImageIcon,
    Folder,
    FolderPlus,
    Pencil,
    ArrowLeft
  } from 'lucide-svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { confirm } from '@tauri-apps/plugin-dialog';
  import { backend } from '$lib/services/backend';
  import type { ImageInfo } from '$lib/types';

  interface ImageWithSrc extends ImageInfo {
    displaySrc: string;
  }

  interface Props {
    open: boolean;
    images: ImageInfo[];
    onSelect?: (image: ImageInfo) => void;
    onDelete?: (image: ImageInfo) => void;
    onUpload?: (folder: string) => void;
    /** Called after folder operations that move or remove images. */
    onImagesChanged?: () => void | Promise<void>;
  }

  interface FolderDialog {
    mode: 'create' | 'rename';
    /** Folder being renamed (rename mode only) */
    path: string;
    value: string;
    error: string;
  }

  let {
    open = $bindable(false),
    images,
    onSelect,
    onDelete,
    onUpload,
    onImagesChanged
  }: Props = $props();

  let searchQuery = $state('');
  let sortBy = $state<'name' | 'date' | 'size'>('date');
  let selectedImage = $state<ImageInfo | null>(null);
  let currentFolder = $state('');
  let folders = $state<string[]>([]);
  let folderDialog = $state<FolderDialog | null>(null);
  let folderBusy = $state(false);

  async function loadFolders() {
    try {
      folders = await backend.listImageFolders();
    } catch (err) {
      console.error('Failed to load image folders:', err);
    }
  }

  $effect(() => {
    if (open) {
      loadFolders();
    } else {
      folderDialog = null;
    }
  });

  const resolveImageSrc = (image: ImageInfo) => {
    if (image.fullPath) return convertFileSrc(image.fullPath);

    const projectPath = backend.getProjectPath();
    if (projectPath) {
      if (image.path) {
        return convertFileSrc(`${projectPath}/source/images/${image.path}`);
      }
      if (image.url && image.url.startsWith('/')) {
        return convertFileSrc(`${projectPath}/source${image.url}`);
      }
    }

    return image.url || '';
  };

  // Pre-compute display URLs for all images
  let imagesWithSrc = $derived(images.map(img => ({
    ...img,
    displaySrc: resolveImageSrc(img)
  })) as ImageWithSrc[]);

  const getImageDir = (image: ImageInfo) => {
    const parts = image.path.split('/').filter(Boolean);
    if (parts.length <= 1) return '';
    return parts.slice(0, -1).join('/');
  };

  let folderSegments = $derived(currentFolder ? currentFolder.split('/').filter(Boolean) : []);

  let folderEntries = $derived((() => {
    const entries = new Map<string, number>();

    // Include folders from disk so empty folders are visible too
    for (const folder of folders) {
      const parentPrefix = currentFolder ? `${currentFolder}/` : '';
      if (!folder.startsWith(parentPrefix)) continue;
      const child = folder.slice(parentPrefix.length).split('/')[0];
      if (child && !entries.has(child)) entries.set(child, 0);
    }

    for (const image of imagesWithSrc) {
      const dir = getImageDir(image);
      if (!dir) continue;

      if (!currentFolder) {
        const child = dir.split('/')[0];
        if (!child) continue;
        entries.set(child, (entries.get(child) ?? 0) + 1);
        continue;
      }

      if (dir.startsWith(`${currentFolder}/`)) {
        const remainder = dir.slice(currentFolder.length + 1);
        const child = remainder.split('/')[0];
        if (child) {
          entries.set(child, (entries.get(child) ?? 0) + 1);
        }
      }
    }

    return Array.from(entries.entries())
      .map(([name, count]) => ({
        name,
        count,
        path: currentFolder ? `${currentFolder}/${name}` : name
      }))
      .sort((a, b) => a.name.localeCompare(b.name));
  })());

  // Filter and sort images
  let filteredImages = $derived(imagesWithSrc
    .filter((img) => {
      const inFolder = getImageDir(img) === currentFolder;
      if (!inFolder) return false;

      if (searchQuery) {
        const query = searchQuery.toLowerCase();
        return (
          img.filename.toLowerCase().includes(query) ||
          img.path.toLowerCase().includes(query)
        );
      }
      return true;
    })
    .sort((a, b) => {
      switch (sortBy) {
        case 'name':
          return a.filename.localeCompare(b.filename);
        case 'date':
          return b.createdAt - a.createdAt;
        case 'size':
          return b.size - a.size;
        default:
          return 0;
      }
    }));

  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
  }

  function handleSelect(image: ImageInfo) {
    selectedImage = image;
    onSelect?.(image);
    open = false;
  }

  async function handleDelete(image: ImageInfo) {
    if (await confirm(`Delete "${image.filename}"?`, { title: 'Delete image', kind: 'warning' })) {
      onDelete?.(image);
    }
  }

  function handleImageClick(image: ImageInfo) {
    selectedImage = image;
  }

  function navigateToFolder(path: string) {
    currentFolder = path;
    selectedImage = null;
  }

  function goUp() {
    if (!currentFolder) return;
    const parts = currentFolder.split('/').filter(Boolean);
    parts.pop();
    currentFolder = parts.join('/');
    selectedImage = null;
  }

  function goToRoot() {
    currentFolder = '';
    selectedImage = null;
  }

  function goToBreadcrumb(index: number) {
    currentFolder = folderSegments.slice(0, index + 1).join('/');
    selectedImage = null;
  }

  function errorMessage(err: unknown): string {
    if (err instanceof Error) return err.message;
    return typeof err === 'string' ? err : 'Unknown error';
  }

  function openCreateFolder() {
    folderDialog = { mode: 'create', path: '', value: '', error: '' };
  }

  function openRenameFolder(path: string) {
    const name = path.split('/').pop() ?? '';
    folderDialog = { mode: 'rename', path, value: name, error: '' };
  }

  async function submitFolderDialog() {
    const dialog = folderDialog;
    if (!dialog || folderBusy) return;

    const name = dialog.value.trim();
    if (!name) {
      dialog.error = 'Folder name is required';
      return;
    }
    if (name.includes('/') || name.includes('\\') || name === '.' || name === '..') {
      dialog.error = 'Folder name must not contain "/" or "\\"';
      return;
    }

    folderBusy = true;
    try {
      if (dialog.mode === 'create') {
        await backend.createImageFolder(currentFolder, name);
        await loadFolders();
      } else {
        const oldName = dialog.path.split('/').pop();
        if (name === oldName) {
          folderDialog = null;
          return;
        }
        const confirmed = await confirm(
          `Rename folder "${oldName}" to "${name}"?\n\n` +
            `Posts and pages that reference images in this folder ` +
            `(e.g. /images/${dialog.path}/...) will have broken links ` +
            `until you update them manually.`,
          { title: 'Rename folder', kind: 'warning', okLabel: 'Rename' }
        );
        if (!confirmed) return;

        await backend.renameImageFolder(dialog.path, name);
        await Promise.all([loadFolders(), onImagesChanged?.()]);
      }
      folderDialog = null;
    } catch (err) {
      dialog.error = errorMessage(err);
    } finally {
      folderBusy = false;
    }
  }

  async function handleDeleteFolder(path: string, imageCount: number) {
    const name = path.split('/').pop();
    try {
      try {
        await backend.deleteImageFolder(path, false);
      } catch (err) {
        if (errorMessage(err) !== 'FOLDER_NOT_EMPTY') throw err;

        const contents =
          imageCount > 0
            ? `${imageCount} image${imageCount === 1 ? '' : 's'}`
            : 'files or subfolders';
        const confirmed = await confirm(
          `Folder "${name}" is not empty. It contains ${contents}.\n\n` +
            `Delete the folder and everything in it? This cannot be undone.`,
          { title: 'Delete folder', kind: 'warning', okLabel: 'Delete' }
        );
        if (!confirmed) return;

        await backend.deleteImageFolder(path, true);
      }
      await Promise.all([loadFolders(), onImagesChanged?.()]);
    } catch (err) {
      console.error('Failed to delete folder:', err);
      alert('Failed to delete folder: ' + errorMessage(err));
    }
  }

  $effect(() => {
    const selected = selectedImage;
    if (!selected) return;
    const stillVisible = filteredImages.some((img) => img.fullPath === selected.fullPath);
    if (!stillVisible) {
      selectedImage = null;
    }
  });
</script>

{#if open}
  <div
    class="modal-overlay"
    onclick={(e) => {
      if (e.currentTarget === e.target) {
        open = false;
      }
    }}
    role="button"
    tabindex="0"
    aria-label="Close image gallery"
    onkeydown={(e) => {
      if (e.currentTarget === e.target && (e.key === 'Enter' || e.key === ' ')) {
        e.preventDefault();
        open = false;
      }
    }}
  >
    <div class="modal-content">
      <!-- Header -->
      <div class="modal-header">
        <h2 class="modal-title">Image Gallery</h2>
        <button
          class="close-btn"
          onclick={() => (open = false)}
          type="button"
          aria-label="Close"
        >
          <X size={24} />
        </button>
      </div>

      <!-- Search and Controls -->
      <div class="modal-controls">
        <div class="search-wrapper">
          <span class="search-icon">
            <Search size={18} />
          </span>
          <input
            type="text"
            class="search-input"
            bind:value={searchQuery}
            placeholder="Search images..."
          />
        </div>

        <div class="controls-right">
          <select class="sort-select" bind:value={sortBy}>
            <option value="date">Sort by date</option>
            <option value="name">Sort by name</option>
            <option value="size">Sort by size</option>
          </select>

          {#if onUpload}
            <button class="upload-btn" onclick={() => onUpload?.(currentFolder)} type="button">
              <UploadIcon size={18} />
              <span>Upload</span>
            </button>
          {/if}
        </div>
      </div>

      <!-- Folder Navigation -->
      <div class="browser-bar">
        <button
          class="folder-up-btn"
          onclick={goUp}
          type="button"
          disabled={!currentFolder}
          title="Go up"
        >
          <ArrowLeft size={16} />
          <span>Up</span>
        </button>
        <div class="breadcrumbs">
          <button class="breadcrumb" onclick={goToRoot} type="button">
            Images
          </button>
          {#each folderSegments as segment, index (segment)}
            <span class="breadcrumb-sep">/</span>
            <button class="breadcrumb" onclick={() => goToBreadcrumb(index)} type="button">
              {segment}
            </button>
          {/each}
        </div>
        <button class="new-folder-btn" onclick={openCreateFolder} type="button">
          <FolderPlus size={16} />
          <span>New folder</span>
        </button>
      </div>

      {#if folderDialog}
        <form
          class="folder-form"
          onsubmit={(e) => {
            e.preventDefault();
            submitFolderDialog();
          }}
        >
          <label class="folder-form-label" for="folder-name-input">
            {folderDialog.mode === 'create' ? 'New folder name' : 'Rename folder to'}
          </label>
          <!-- svelte-ignore a11y_autofocus -->
          <input
            id="folder-name-input"
            class="folder-form-input"
            type="text"
            bind:value={folderDialog.value}
            oninput={() => folderDialog && (folderDialog.error = '')}
            onkeydown={(e) => {
              if (e.key === 'Escape') {
                e.preventDefault();
                e.stopPropagation();
                folderDialog = null;
              }
            }}
            disabled={folderBusy}
            autofocus
          />
          <button class="folder-form-submit" type="submit" disabled={folderBusy}>
            {folderDialog.mode === 'create' ? 'Create' : 'Rename'}
          </button>
          <button
            class="folder-form-cancel"
            type="button"
            onclick={() => (folderDialog = null)}
            disabled={folderBusy}
          >
            Cancel
          </button>
          {#if folderDialog.error}
            <p class="folder-form-error">{folderDialog.error}</p>
          {/if}
        </form>
      {/if}

      <!-- Images Grid -->
      <div class="images-grid">
        {#if !searchQuery && folderEntries.length > 0}
          {#each folderEntries as folder (folder.path)}
            <div
              class="folder-card"
              onclick={() => navigateToFolder(folder.path)}
              role="button"
              tabindex="0"
              onkeydown={(e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                  e.preventDefault();
                  navigateToFolder(folder.path);
                }
              }}
            >
              <div class="folder-thumb">
                <Folder size={32} />
              </div>
              <div class="folder-info">
                <p class="folder-name" title={folder.name}>{folder.name}</p>
                <p class="folder-meta">{folder.count} image{folder.count === 1 ? '' : 's'}</p>
              </div>
              <div class="folder-actions">
                <button
                  class="folder-action-btn"
                  onclick={(e) => {
                    e.stopPropagation();
                    openRenameFolder(folder.path);
                  }}
                  type="button"
                  aria-label="Rename folder"
                  title="Rename folder"
                >
                  <Pencil size={14} />
                </button>
                <button
                  class="folder-action-btn danger"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDeleteFolder(folder.path, folder.count);
                  }}
                  type="button"
                  aria-label="Delete folder"
                  title="Delete folder"
                >
                  <Trash2 size={14} />
                </button>
              </div>
            </div>
          {/each}
        {/if}
        {#each filteredImages as image (image.fullPath || image.path || image.filename)}
          <div
            class="image-card"
            class:selected={selectedImage?.fullPath === image.fullPath}
            onclick={() => handleImageClick(image)}
            ondblclick={() => handleSelect(image)}
            role="button"
            tabindex="0"
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                handleSelect(image);
              }
            }}
          >
            <div class="image-thumb">
              {#if image.displaySrc}
                <img
                  src={image.displaySrc}
                  alt={image.filename}
                  loading="lazy"
                  onerror={(e) => {
                    // Hide broken image and show placeholder
                    const img = e.target as HTMLImageElement;
                    img.style.display = 'none';
                    const placeholder = img.nextElementSibling as HTMLElement | null;
                    if (placeholder) placeholder.style.display = 'flex';
                  }}
                />
                <div class="no-image error-placeholder" style="display: none;">
                  <ImageIcon size={32} />
                </div>
              {:else}
                <div class="no-image">
                  <ImageIcon size={32} />
                </div>
              {/if}
            </div>
            <div class="image-info">
              <p class="image-name" title={image.filename}>{image.filename}</p>
              <p class="image-meta">{formatBytes(image.size)}</p>
              {#if onDelete}
                <button
                  class="delete-btn"
                  onclick={(e) => {
                    e.stopPropagation();
                    handleDelete(image);
                  }}
                  type="button"
                  aria-label="Delete image"
                >
                  <Trash2 size={14} />
                </button>
              {/if}
            </div>
          </div>
        {/each}

        {#if filteredImages.length === 0 && folderEntries.length === 0}
          <div class="empty-gallery">
            <span class="empty-icon">
              <ImageIcon size={48} />
            </span>
            <h3>No images found</h3>
            <p>
              {#if searchQuery}
                Try a different search term
              {:else if currentFolder}
                This folder is empty
              {:else if onUpload}
                Upload an image to get started
              {:else}
                No images available
              {/if}
            </p>
            {#if onUpload && !searchQuery}
              <button class="upload-btn-empty" onclick={() => onUpload?.(currentFolder)} type="button">
                <UploadIcon size={18} />
                Upload Image
              </button>
            {/if}
          </div>
        {/if}
      </div>

      <!-- Selected Image Info -->
      {#if selectedImage}
        <div class="selected-info">
          <p class="info-text">
            Selected: <strong>{selectedImage.filename}</strong> ({formatBytes(selectedImage.size)})
          </p>
          <button
            class="select-btn"
            onclick={() => selectedImage && handleSelect(selectedImage)}
            type="button"
          >
            Insert
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: 1rem;
  }

  .modal-content {
    width: 100%;
    max-width: 900px;
    max-height: 80vh;
    background-color: #ffffff;
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  :global(.dark .modal-content) {
    background-color: #2d2d2d;
  }

  /* Header */
  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e5e5e5;
  }

  :global(.dark .modal-header) {
    border-bottom-color: #404040;
  }

  .modal-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: #1a1a1a;
    margin: 0;
  }

  :global(.dark .modal-title) {
    color: #f5f5f5;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.25rem;
    background-color: transparent;
    border: none;
    color: #666666;
    cursor: pointer;
    border-radius: 0.25rem;
  }

  :global(.dark .close-btn) {
    color: #a3a3a3;
  }

  .close-btn:hover {
    background-color: #f7f7f7;
  }

  :global(.dark .close-btn:hover) {
    background-color: #404040;
  }

  /* Controls */
  .modal-controls {
    display: flex;
    gap: 1rem;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e5e5e5;
  }

  :global(.dark .modal-controls) {
    border-bottom-color: #404040;
  }

  .search-wrapper {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 0.75rem;
    color: #9ca3af;
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 0.625rem 1rem 0.625rem 2.5rem;
    background-color: #ffffff;
    border: 1px solid #e5e5e5;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    color: #1a1a1a;
  }

  :global(.dark .search-input) {
    background-color: #404040;
    border-color: #525252;
    color: #f5f5f5;
  }

  .search-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .controls-right {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .sort-select {
    padding: 0.625rem 0.75rem;
    background-color: #ffffff;
    border: 1px solid #e5e5e5;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  :global(.dark .sort-select) {
    background-color: #404040;
    border-color: #525252;
  }

  .upload-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .upload-btn:hover {
    background-color: #2563eb;
  }

  /* Folder Browser */
  .browser-bar {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    border-bottom: 1px solid #e5e5e5;
    background-color: #fafafa;
  }

  :global(.dark .browser-bar) {
    border-bottom-color: #404040;
    background-color: #2d2d2d;
  }

  .folder-up-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.65rem;
    border-radius: 0.375rem;
    border: 1px solid #e5e5e5;
    background-color: #ffffff;
    color: #374151;
    cursor: pointer;
    font-size: 0.75rem;
  }

  :global(.dark .folder-up-btn) {
    background-color: #404040;
    border-color: #525252;
    color: #e5e7eb;
  }

  .folder-up-btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .new-folder-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: auto;
    padding: 0.35rem 0.65rem;
    border-radius: 0.375rem;
    border: 1px solid #e5e5e5;
    background-color: #ffffff;
    color: #374151;
    cursor: pointer;
    font-size: 0.75rem;
  }

  .new-folder-btn:hover {
    border-color: #bfdbfe;
  }

  :global(.dark .new-folder-btn) {
    background-color: #404040;
    border-color: #525252;
    color: #e5e7eb;
  }

  .folder-form {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    border-bottom: 1px solid #e5e5e5;
  }

  :global(.dark .folder-form) {
    border-bottom-color: #404040;
  }

  .folder-form-label {
    font-size: 0.8125rem;
    color: #374151;
  }

  :global(.dark .folder-form-label) {
    color: #e5e7eb;
  }

  .folder-form-input {
    flex: 1;
    min-width: 160px;
    padding: 0.4rem 0.6rem;
    background-color: #ffffff;
    border: 1px solid #e5e5e5;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    color: #1a1a1a;
  }

  .folder-form-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  :global(.dark .folder-form-input) {
    background-color: #404040;
    border-color: #525252;
    color: #f5f5f5;
  }

  .folder-form-submit,
  .folder-form-cancel {
    padding: 0.4rem 0.9rem;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    cursor: pointer;
  }

  .folder-form-submit {
    background-color: #3b82f6;
    color: white;
    border: none;
  }

  .folder-form-submit:hover {
    background-color: #2563eb;
  }

  .folder-form-cancel {
    background-color: transparent;
    border: 1px solid #e5e5e5;
    color: #374151;
  }

  :global(.dark .folder-form-cancel) {
    border-color: #525252;
    color: #e5e7eb;
  }

  .folder-form-submit:disabled,
  .folder-form-cancel:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .folder-form-error {
    flex-basis: 100%;
    margin: 0;
    font-size: 0.75rem;
    color: #dc2626;
  }

  .folder-actions {
    position: absolute;
    top: 0.35rem;
    right: 0.35rem;
    display: flex;
    gap: 0.25rem;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .folder-card:hover .folder-actions,
  .folder-card:focus-within .folder-actions {
    opacity: 1;
  }

  .folder-action-btn {
    display: flex;
    padding: 0.25rem;
    background-color: rgba(255, 255, 255, 0.9);
    border: none;
    border-radius: 0.25rem;
    color: #374151;
    cursor: pointer;
  }

  .folder-action-btn.danger {
    color: #dc2626;
  }

  .folder-action-btn:hover {
    background-color: #ffffff;
  }

  :global(.dark .folder-action-btn) {
    background-color: rgba(0, 0, 0, 0.8);
    color: #e5e7eb;
  }

  :global(.dark .folder-action-btn.danger) {
    color: #f87171;
  }

  .breadcrumbs {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .breadcrumb {
    background: none;
    border: none;
    color: #2563eb;
    font-size: 0.8125rem;
    cursor: pointer;
    padding: 0;
  }

  :global(.dark .breadcrumb) {
    color: #93c5fd;
  }

  .breadcrumb-sep {
    color: #9ca3af;
    font-size: 0.75rem;
  }

  :global(.dark .breadcrumb-sep) {
    color: #6b7280;
  }

  /* Images Grid */
  .images-grid {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 1.5rem;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 1rem;
  }

  .folder-card {
    position: relative;
    cursor: pointer;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    overflow: hidden;
    transition: all 0.15s ease;
    height: 100px;
    background-color: #ffffff;
    display: flex;
    flex-direction: column;
  }

  :global(.dark .folder-card) {
    background-color: #2d2d2d;
  }

  .folder-card:hover {
    border-color: #bfdbfe;
  }

  :global(.dark .folder-card:hover) {
    border-color: #1e40af;
  }

  .folder-thumb {
    flex: 1;
    background-color: #f7f7f7;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #3b82f6;
  }

  :global(.dark .folder-thumb) {
    background-color: #404040;
    color: #93c5fd;
  }

  .folder-info {
    padding: 0.5rem;
    border-top: 1px solid #e5e5e5;
  }

  :global(.dark .folder-info) {
    border-top-color: #404040;
  }

  .folder-name {
    font-size: 0.75rem;
    font-weight: 600;
    color: #1a1a1a;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark .folder-name) {
    color: #f5f5f5;
  }

  .folder-meta {
    font-size: 0.714rem;
    color: #666666;
    margin: 0;
  }

  :global(.dark .folder-meta) {
    color: #a3a3a3;
  }

  .image-card {
    position: relative;
    cursor: pointer;
    border: 2px solid transparent;
    border-radius: 0.5rem;
    overflow: hidden;
    transition: all 0.15s ease;
    height: 100px;
  }

  .image-card:hover {
    border-color: #bfdbfe;
  }

  :global(.dark .image-card:hover) {
    border-color: #1e40af;
  }

  .image-card.selected {
    border-color: #3b82f6;
  }

  :global(.dark .image-card.selected) {
    border-color: #3b82f6;
  }

  .image-thumb {
    aspect-ratio: 1;
    min-height: 120px;
    height: 100px;
    background-color: #f7f7f7;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    position: relative;
  }

  :global(.dark .image-thumb) {
    background-color: #404040;
  }

  .image-thumb img {
    height: 100%;
    width: 100%;
    object-fit: cover;
  }

  .no-image {
    color: #d1d5db;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
  }

  .no-image.error-placeholder {
    background-color: #f7f7f7;
  }

  :global(.dark .no-image) {
    color: #525252;
  }

  :global(.dark .no-image.error-placeholder) {
    background-color: #404040;
  }

  .image-info {
    padding: 0.5rem;
    background-color: #ffffff;
    border-top: 1px solid #e5e5e5;
  }

  :global(.dark .image-info) {
    background-color: #2d2d2d;
    border-top-color: #404040;
  }

  .image-name {
    font-size: 0.75rem;
    font-weight: 500;
    color: #1a1a1a;
    margin: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.dark .image-name) {
    color: #f5f5f5;
  }

  .image-meta {
    font-size: 0.714px;
    color: #666666;
    margin: 0;
  }

  :global(.dark .image-meta) {
    color: #a3a3a3;
  }

  .delete-btn {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    padding: 0.25rem;
    background-color: rgba(255, 255, 255, 0.9);
    border: none;
    border-radius: 0.25rem;
    color: #dc2626;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .image-card:hover .delete-btn {
    opacity: 1;
  }

  .delete-btn:hover {
    background-color: #ffffff;
  }

  :global(.dark .delete-btn) {
    background-color: rgba(0, 0, 0, 0.8);
  }

  /* Empty Gallery */
  .empty-gallery {
    grid-column: 1 / -1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    text-align: center;
  }

  .empty-icon {
    color: #d1d5db;
    margin-bottom: 1rem;
  }

  :global(.dark .empty-icon) {
    color: #525252;
  }

  .empty-gallery h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #1a1a1a;
    margin: 0 0 0.5rem 0;
  }

  :global(.dark .empty-gallery h3) {
    color: #f5f5f5;
  }

  .empty-gallery p {
    font-size: 0.875rem;
    color: #666666;
    margin: 0 0 1rem 0;
  }

  :global(.dark .empty-gallery p) {
    color: #a3a3a3;
  }

  .upload-btn-empty {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .upload-btn-empty:hover {
    background-color: #2563eb;
  }

  /* Selected Info */
  .selected-info {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    border-top: 1px solid #e5e5e5;
  }

  :global(.dark .selected-info) {
    border-top-color: #404040;
  }

  .info-text {
    font-size: 0.875rem;
    color: #1a1a1a;
    margin: 0;
  }

  :global(.dark .info-text) {
    color: #f5f5f5;
  }

  .select-btn {
    padding: 0.625rem 1.5rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
  }

  .select-btn:hover {
    background-color: #2563eb;
  }
</style>
