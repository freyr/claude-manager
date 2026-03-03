# Changelog

## 0.4.0

### Features

- **Compose screen** — new screen (`3` key) for assembling CLAUDE.md files from your snippet library. Toggle-select snippets with `Space`, see a live preview in the right pane, and export to a file with `w`. Snippets are composed in selection order, so selecting C then A produces C followed by A in the output.
- **Library screen** — the snippet library is now a top-level screen (`4` key) with a left/right dual-pane layout (snippet list + content preview), replacing the previous overlay mode accessed via `L`.
- **Edit snippets** — press `e` on the Library screen to edit a snippet's content in the text editor. Changes are saved back to `library.toml` with `Ctrl+S`.
- **Configuration file** — `~/.config/jigolo/config.toml` supports `theme`, `default_paths`, and `default_depth` settings.
- **Color themes** — press `T` to toggle between dark and light themes.
- **Settings fold/unfold** — press `←`/`→` to collapse and expand settings sections.
- **Effective settings view** — press `m` on the Settings screen to see the merged effective settings across all config files.

### Improvements

- **Four-tab navigation** — tab bar shows `[1 Files] [2 Settings] [3 Compose] [4 Library]`; number keys switch screens from anywhere.
- **Help bar cleanup** — bottom help bar now shows only action keybindings (arrow key symbols instead of vim-style `j/k`); screen navigation hints removed since the tab bar already shows them.

### Internal

- **Module split** — `app.rs` (3,865 lines) split into per-screen modules: `files.rs`, `settings.rs`, `edit.rs`, `compose.rs`, `library.rs`. Core `app.rs` reduced to ~700 lines.

## 0.3.1

### Features

- **Scan depth limit** — default scan depth reduced from 100 to 3 levels for faster directory traversal; override with `--depth N` when deeper scanning is needed

### Fixes

- **Folder navigation** — folder nodes are now selectable in the file tree; the content pane clears when a folder is selected instead of showing stale file content
- **Left arrow on folders** — pressing Left on a closed folder no longer causes the cursor to disappear
- **Enter key removed** — removed the misleading Enter/"Open" keybinding from the file tree since it duplicated existing arrow key behavior
- **Cursor visibility** — replaced underline cursor with reversed (inverted) style in the content and settings panes so the cursor is visible on empty lines
- **IO warning noise** — broken symlinks, permission errors, and inaccessible files (e.g. Google Drive placeholders) are now silently skipped instead of printing warnings to stderr

## 0.3.0

### Features
    
- **In-place text editing** — press `e` to edit CLAUDE.md files and settings files directly in the TUI using tui-textarea, with dirty tracking, explicit save (`Ctrl+S`), atomic writes, and double-`Esc` to discard unsaved changes

### Fixes

- **File list navigation** — `j`/`k` now skip folder nodes so the cursor always lands on a file, preventing the content pane from appearing unresponsive

### Internal

- **Crate structure** — moved business logic from `main.rs` to `lib.rs`, eliminating duplicate module compilation during tests

## 0.2.0

### Features

- **Settings viewer** — new screen (`2` key) discovers and displays Claude Code settings files (`~/.claude/settings.json`, project settings, project local settings) in a structured, scrollable format showing model, permissions, MCP servers, hooks, plugins, and environment variables
- **Screen switching** — tab bar with `1` / `2` keys to switch between Files and Settings screens

### Improvements

- **Help bar readability** — key labels now use cyan and descriptions use gray for better visibility on dark terminals
- **Text input cursor** — arrow keys move the cursor in title and rename inputs; backspace deletes at cursor position
- **Rename pre-fill** — renaming a snippet now places the cursor at the end of the existing title

### Internal

- **Project renamed** from context-manager to jigolo — updated package name, binary, repo URLs, homebrew tap, config directory, and all references

## 0.1.0

Initial release.

### Features

- **Dual-pane TUI browser** — left pane shows a tree of discovered CLAUDE.md files, right pane displays content with vim-style cursor navigation (j/k, PageUp/PageDown)
- **Recursive discovery** — walks directory trees finding CLAUDE.md files, skips node_modules/.git/target and other noise directories
- **Global CLAUDE.md** — auto-discovers `~/.claude/CLAUDE.md` and prepends it to the file list
- **Snippet capture** — visual line selection (`v`), title input (`s`), saves to `~/.config/jigolo/library.toml`
- **Library browser** — press `L` to browse saved snippets with split list/preview pane
- **Snippet management** — rename (`r`) and delete (`d`) snippets from the library browser
- **Context-sensitive help bar** — bottom bar shows available keybindings for the current mode
- **List mode** — `--list` flag prints discovered files and exits (no TUI)

### Keybindings

| Key | Action |
|-----|--------|
| `j/k` | Navigate / scroll |
| `Tab` | Switch pane |
| `Enter` | Open/select |
| `v` | Start visual selection |
| `s` | Save selection as snippet |
| `L` | Open library browser |
| `r` | Rename snippet (in library) |
| `d` | Delete snippet (in library) |
| `Esc` | Cancel / go back |
| `q` | Quit |
