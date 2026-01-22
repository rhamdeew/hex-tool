# Hex Tool

Hex Tool is a desktop GUI editor for Hexo blogs built with Tauri, SvelteKit, and TypeScript. It provides a fast, native-feeling workflow for editing posts and managing content while keeping the project structure on disk.

![Hex Tool screenshot](https://github.com/user-attachments/assets/1ca89750-5421-4e50-b1c0-ccf6ec3f36f4)

## Usage

1. Download a build for your platform from the "Releases" page.
2. Launch the app and select your Hexo project folder (the one with `package.json` and `source/`).
3. Manage posts, pages, and drafts, then edit content with live frontmatter controls.
4. Use the Hexo controls to run `hexo server`, `hexo generate`, `hexo clean`, and `hexo deploy` from the UI.

### Fixing macOS Gatekeeper Issue

If macOS prevents you from opening the app with a message like **"Hex Tool" cannot be opened because the developer cannot be verified**, you need to remove the quarantine attribute that macOS applies to downloaded applications.

Run this command in Terminal:

```sh
xattr -dr com.apple.quarantine "/Applications/Hex Tool.app"
```

This removes the quarantine flag and allows the app to run. You only need to do this once after installing Hex Tool.

### Requirements for Hexo Commands

Hexo commands run through `npx hexo`, so your blog should have dependencies installed:

```sh
cd /path/to/your/hexo/blog
npm install
```

If you only edit Markdown files, the app still works without running Hexo commands.

### Frontmatter Configuration (Optional)

Hex Tool supports custom frontmatter fields via `frontmatter-config.json` in the project root. You can generate a starter config from existing posts using the "Generate frontmatter config" action in the app.

## Development

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Prerequisites

- Node.js (LTS recommended)
- Rust toolchain (`rustup`)
- Tauri system dependencies: https://tauri.app/start/prerequisites/

### Getting Started

Install dependencies:

```sh
npm install
```

Run the desktop app in development mode:

```sh
npm run dev:app
```

Run frontend-only development:

```sh
npm run dev
```

### Key Commands

- `npm run build`: build the frontend.
- `npm run build:tauri`: build the full Tauri app.
- `npm run check`: run Svelte/TypeScript checks.
- `npm run lint`: run ESLint.
- `npm run format`: format code with Prettier.
- `npm run test`: run frontend unit tests.
- `make cargo-check`: check the Rust backend in `src-tauri/`.
- `make cargo-clippy`: lint Rust code.
- `make cargo-test`: run Rust tests.

### Project Layout

- `src/`: SvelteKit frontend (`routes/` and `lib/`).
- `src-tauri/`: Rust backend and Tauri configuration.
- `static/`: static assets served by SvelteKit.
- `example_blog/`: sample Hexo blog for local testing.
