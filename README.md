# Jigolo

Your [Claude Code](https://claude.com/claude-code) context files are scattered across dozens of projects and directories. Jigolo finds them all, lets you read them side by side, and helps you build better ones.

It is a terminal app that recursively discovers every `CLAUDE.md` in your directory trees, displays them in a fast dual-pane browser, and gives you tools to work with them: inspect Claude Code settings across all your projects, save the best rules and patterns to a personal snippet library, and compose new `CLAUDE.md` files by picking and assembling snippets from that library.

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

Press `2` to inspect your Claude Code settings across all configuration layers:

- `~/.claude/settings.json` (Global)
- `.claude/settings.json` (Project)
- `.claude/settings.local.json` (Project Local)

Displays model, permissions, MCP servers, hooks, plugins, and environment variables in a structured, scrollable format. Use `←`/`→` to fold and unfold sections, and `m` to toggle the merged effective settings view that shows the final resolved values after all layers are combined.

### Snippet Library

Build a personal library of reusable CLAUDE.md rules and patterns:

1. Navigate to a CLAUDE.md file and switch to the content pane (`Tab`)
2. Move cursor to the start line
3. Press `v` to begin visual selection
4. Extend selection with arrow keys
5. Press `s`, type a title, press `Enter`

Snippets are saved to `~/.config/jigolo/library.toml`. Press `4` to open the Library screen, where you can browse all saved snippets in a dual-pane view (titles on the left, content on the right). From there you can edit (`e`) a snippet's content, rename (`r`) its title, or delete (`d`) it.

### Compose Mode

Assemble new CLAUDE.md files by picking snippets from your library:

1. Press `3` to open the Compose screen
2. Navigate the snippet list and press `Space` to select (each selection appends to the composed output)
3. Review the live preview in the right pane as you build up the file
4. Press `w`, type a file path, and press `Enter` to export

Snippets appear in the output in the order you select them, so you control the structure of the resulting file. Deselecting a snippet removes it from the preview.

### Configuration

Jigolo stores its configuration at `~/.config/jigolo/config.toml`:

```toml
theme = "dark"              # "dark" or "light"
default_paths = ["/path1"]  # directories to scan on startup
default_depth = 3           # max directory depth (default: 3)
```

All settings are optional. CLI arguments override config file values.

## License

MIT
