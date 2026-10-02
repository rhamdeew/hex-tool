// Backend service for Tauri command invocations
import { invoke } from '@tauri-apps/api/core';
import type {
  Post,
  Page,
  Draft,
  ImageInfo,
  HexoConfig,
  Frontmatter,
  FrontmatterConfig,
  AppConfig,
  CommandOutput
} from '$lib/types';

export class BackendService {
  private projectPath: string | null = null;

  constructor() {
    this.loadProjectPath();
  }

  private loadProjectPath() {
    this.projectPath = localStorage.getItem('projectPath') || null;
  }

  setProjectPath(path: string) {
    this.projectPath = path;
    localStorage.setItem('projectPath', path);
  }

  getProjectPath(): string | null {
    return this.projectPath;
  }

  private ensureProject(): string {
    if (!this.projectPath) {
      throw new Error('No project selected. Please select a project folder first.');
    }
    return this.projectPath;
  }

  // ====================
  // Project Commands
  // ====================

  async selectProjectFolder(): Promise<string> {
    const path = await invoke<string>('select_project_folder');
    this.setProjectPath(path);
    return path;
  }

  async activateProject(path: string): Promise<void> {
    await invoke('activate_project', { projectPath: path });
    this.setProjectPath(path);
  }

  /** Re-grants asset access for the project path persisted from a previous session. */
  async restoreProject(): Promise<void> {
    if (!this.projectPath) return;
    try {
      await invoke('activate_project', { projectPath: this.projectPath });
    } catch (err) {
      console.error('Failed to restore project access:', err);
    }
  }

  async getProjectConfig(): Promise<HexoConfig> {
    const projectPath = this.ensureProject();
    return invoke<HexoConfig>('get_project_config', { projectPath });
  }

  async getFrontmatterConfig(): Promise<FrontmatterConfig> {
    const projectPath = this.ensureProject();
    return invoke<FrontmatterConfig>('get_frontmatter_config', { projectPath });
  }

  async generateFrontmatterConfig(): Promise<FrontmatterConfig> {
    const projectPath = this.ensureProject();
    return invoke<FrontmatterConfig>('generate_frontmatter_config_command', { projectPath });
  }

  async getFrontmatterConfigRaw(): Promise<string | null> {
    const projectPath = this.ensureProject();
    return invoke<string | null>('get_frontmatter_config_raw', { projectPath });
  }

  async saveFrontmatterConfigRaw(content: string): Promise<FrontmatterConfig> {
    const projectPath = this.ensureProject();
    return invoke<FrontmatterConfig>('save_frontmatter_config_raw', { projectPath, content });
  }

  async serializeFrontmatter(frontmatter: Frontmatter): Promise<string> {
    return invoke<string>('serialize_frontmatter', { frontmatter });
  }

  async parseFrontmatter(yaml: string): Promise<Frontmatter> {
    return invoke<Frontmatter>('parse_frontmatter', { yaml });
  }

  // ====================
  // Posts Commands
  // ====================

  async listPosts(): Promise<Post[]> {
    const projectPath = this.ensureProject();
    return invoke<Post[]>('list_posts', { projectPath });
  }

  async getPost(postId: string): Promise<Post> {
    const projectPath = this.ensureProject();
    return invoke<Post>('get_post', { projectPath, postId });
  }

  async savePost(post: Post): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('save_post', { projectPath, post });
  }

  async getPage(pageId: string): Promise<Page> {
    const projectPath = this.ensureProject();
    return invoke<Page>('get_page', { projectPath, pageId });
  }

  async savePage(page: Page): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('save_page', { projectPath, page });
  }

  async deletePage(pageId: string): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('delete_page', { projectPath, pageId });
  }

  async createPost(title: string): Promise<Post> {
    const projectPath = this.ensureProject();
    return invoke<Post>('create_post', { projectPath, title });
  }

  async deletePost(postId: string): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('delete_post', { projectPath, postId });
  }

  // ====================
  // Pages Commands
  // ====================

  async createPage(title: string): Promise<Page> {
    const projectPath = this.ensureProject();
    return invoke<Page>('create_page', { projectPath, title });
  }

  async listPages(): Promise<Page[]> {
    const projectPath = this.ensureProject();
    return invoke<Page[]>('list_pages', { projectPath });
  }

  // ====================
  // Drafts Commands
  // ====================

  async createDraft(title: string): Promise<Draft> {
    const projectPath = this.ensureProject();
    return invoke<Draft>('create_draft', { projectPath, title });
  }

  async getDraft(draftId: string): Promise<Draft> {
    const projectPath = this.ensureProject();
    return invoke<Draft>('get_draft', { projectPath, draftId });
  }

  async saveDraft(draft: Draft): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('save_draft', { projectPath, draft });
  }

  async deleteDraft(draftId: string): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('delete_draft', { projectPath, draftId });
  }

  async listDrafts(): Promise<Draft[]> {
    const projectPath = this.ensureProject();
    return invoke<Draft[]>('list_drafts', { projectPath });
  }

  // ====================
  // Images Commands
  // ====================

  async listImages(): Promise<ImageInfo[]> {
    const projectPath = this.ensureProject();
    return invoke<ImageInfo[]>('list_images', { projectPath });
  }

  async copyImageToProject(sourcePath: string, subfolder: string = ''): Promise<string> {
    const projectPath = this.ensureProject();
    return invoke<string>('copy_image_to_project', { projectPath, sourcePath, subfolder });
  }

  async deleteImage(imagePath: string): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('delete_image', { projectPath, imagePath });
  }

  async listImageFolders(): Promise<string[]> {
    const projectPath = this.ensureProject();
    return invoke<string[]>('list_image_folders', { projectPath });
  }

  async createImageFolder(parent: string, name: string): Promise<string> {
    const projectPath = this.ensureProject();
    return invoke<string>('create_image_folder', { projectPath, parent, name });
  }

  async renameImageFolder(folder: string, newName: string): Promise<string> {
    const projectPath = this.ensureProject();
    return invoke<string>('rename_image_folder', { projectPath, folder, newName });
  }

  /** Rejects with `FOLDER_NOT_EMPTY` when `recursive` is false and the folder has contents. */
  async deleteImageFolder(folder: string, recursive: boolean): Promise<void> {
    const projectPath = this.ensureProject();
    await invoke('delete_image_folder', { projectPath, folder, recursive });
  }

  // ====================
  // App Config Commands
  // ====================

  async getAppConfig(): Promise<AppConfig> {
    return invoke<AppConfig>('get_app_config');
  }

  async saveAppConfig(config: AppConfig): Promise<void> {
    await invoke('save_app_config', { config });
  }

  // ====================
  // Hexo Server Commands
  // ====================

  async runHexoCommand(command: string): Promise<CommandOutput> {
    const projectPath = this.ensureProject();
    return invoke<CommandOutput>('run_hexo_command', { projectPath, command });
  }

  async startHexoServer(): Promise<string> {
    const projectPath = this.ensureProject();
    return invoke<string>('start_hexo_server', { projectPath });
  }

  async stopHexoServer(serverId: string): Promise<void> {
    await invoke('stop_hexo_server', { serverId });
  }

  async isHexoServerRunning(): Promise<boolean> {
    const projectPath = this.ensureProject();
    return invoke<boolean>('is_hexo_server_running', { projectPath });
  }
}

// Singleton instance
export const backend = new BackendService();
