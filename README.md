# Jigolo

A TUI for browsing and managing [Claude Code](https://claude.com/claude-code) context files (`CLAUDE.md`). Discover files across directory trees, read them in a dual-pane browser, and build a personal snippet library of reusable rules and patterns.

## Installation

### Homebrew

```sh
brew install freyr/tap/jigolo
```

### From source

```sh
cargo install --path .
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/freyr/jigolo/releases).

Shell installer (macOS/Linux):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/freyr/jigolo/releases/latest/download/jigolo-installer.sh | sh
```

## Usage

```sh
jigolo                    # Browse current directory
jigolo /path1 /path2     # Browse specific directories
jigolo --list /path      # List files and exit (no TUI)
```

The TUI has four screens, switched with number keys:

| Key | Screen | Purpose |
|-----|--------|---------|
| `1` | **Files** | Dual-pane browser with file tree (left) and content (right) |
| `2` | **Settings** | Claude Code settings viewer with fold/unfold and merged view |
| `3` | **Compose** | Assemble new CLAUDE.md files from your snippet library |
| `4` | **Library** | Browse, edit, rename, and delete saved snippets |

### Keybindings

**Global** (work on any screen in Normal mode):

| Key | Action |
|-----|--------|
| `1` / `2` / `3` / `4` | Switch screen |
| `T` | Toggle dark/light theme |
| `Esc` | Go back |
| `q` | Quit |

**Files screen:**

| Key | Action |
|-----|--------|
| `Tab` | Switch pane (tree / content) |
| `v` | Start visual line selection |
| `s` | Save selection as snippet |
| `e` | Edit file |

**Compose screen:**

| Key | Action |
|-----|--------|
| `Space` | Toggle snippet selection (appends to composed output) |
| `Tab` | Switch focus (snippet list / preview) |
| `w` | Export composed output to file |

**Library screen:**

| Key | Action |
|-----|--------|
| `e` | Edit snippet content |
| `r` | Rename snippet |
| `d` | Delete snippet |

### Settings Viewer

Press `2` to view Claude Code settings from all discovered files:

- `~/.claude/settings.json` (Global)
- `.claude/settings.json` (Project)
- `.claude/settings.local.json` (Project Local)

Displays model, permissions, MCP servers, hooks, plugins, and environment variables in a structured format. Use `←`/`→` to fold/unfold sections and `m` to toggle the merged effective settings view.

### Snippet Library

Select text you want to reuse across projects:

1. Navigate to a CLAUDE.md file and switch to the content pane (`Tab`)
2. Move cursor to the start line
3. Press `v` to begin visual selection
4. Extend selection with arrow keys
5. Press `s`, type a title, press `Enter`

Snippets are saved to `~/.config/jigolo/library.toml`. Press `4` to browse the library, where you can edit (`e`), rename (`r`), or delete (`d`) saved snippets.

### Compose Mode

Assemble new CLAUDE.md files from your snippet library:

1. Press `3` to open the Compose screen
2. Select snippets with `Space` (each selection appends to the preview)
3. Review the live preview in the right pane
4. Press `w` to export to a file

## License

MIT
