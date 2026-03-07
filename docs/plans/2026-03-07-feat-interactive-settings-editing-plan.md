# Interactive Settings Editing Plan (#33)

## Goal

Add interactive, structured editing to the Settings screen so users can toggle booleans, add/remove permission entries, and manage MCP servers without editing raw JSON.

## Architecture

### Data Model

The existing `SettingsState` tracks display lines and a `line_map` to source files. We add a **semantic model** that maps each display line to a typed `SettingsEntry`:

```rust
enum SettingsEntry {
    SectionHeader { file_idx: usize },
    BooleanField { file_idx: usize, key: String, value: bool },
    ScalarField { file_idx: usize, key: String, value: String },
    PermissionHeader { file_idx: usize, category: String },
    PermissionItem { file_idx: usize, category: String, value: String },
    McpServerHeader { file_idx: usize },
    McpServer { file_idx: usize, name: String },
    SubHeader { file_idx: usize, key: String },
    Leaf { file_idx: usize },
    Blank,
}
```

A `Vec<SettingsEntry>` parallel to `lines` enables type-aware key handling.

### Mutation Flow

1. User presses an action key (Space to toggle, `d` to delete, `a` to add)
2. Handler reads the `SettingsEntry` at cursor to determine the action
3. Modifies the in-memory `serde_json::Value` in the `SettingsCollection`
4. Writes the modified JSON back to the source file (atomic write)
5. Rebuilds the display lines from the updated collection

### Files Changed

- `src/settings.rs` — Add `SettingsEntry` enum, `build_entry_map()` function, `write_settings_file()` for atomic save
- `src/tui/app.rs` — Add `entry_map: Vec<SettingsEntry>` to `SettingsState`
- `src/tui/settings.rs` — Wire new keys (Space, d, a) to entry-aware handlers, update help bar

## Implementation Steps

### Step 1: SettingsEntry enum and entry map builder

Add `SettingsEntry` to `src/settings.rs` alongside existing formatting code. Build the map during `format_settings_with_map()` so entries and lines stay in sync.

### Step 2: Wire entry map into SettingsState

Store `entry_map` in `SettingsState`, populate during `rebuild_settings_display()`.

### Step 3: Atomic JSON write-back

Add `write_settings_file(path, value)` that pretty-prints JSON and atomically writes via tempfile.

### Step 4: Toggle boolean fields (Space key)

When cursor is on a `BooleanField`, Space flips the value, writes back, and rebuilds display.

### Step 5: Delete permission items (d key)

When cursor is on a `PermissionItem`, `d` removes it from the array, writes back, rebuilds.

### Step 6: Add permission items (a key)

When cursor is on a `PermissionHeader` or `PermissionItem`, `a` opens a text input to type a new entry. On Enter, appends to the permission array.

### Step 7: Update help bar

Show context-sensitive hints based on the entry type at cursor.

### Step 8: MCP server toggle/remove

When cursor is on an `McpServer`, Space toggles enabled/disabled (by adding/removing from the object), `d` removes entirely.
