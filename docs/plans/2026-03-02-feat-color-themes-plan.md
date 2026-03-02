---
title: "feat: Color themes"
type: feat
date: 2026-03-02
issue: 7
depends_on: []
---

# feat: Color themes (#7)

## Overview

Extract all hardcoded color and style definitions from `src/tui/app.rs` into a centralized `Theme` struct, provide built-in dark and light palettes, and add a runtime toggle (`T`) to switch between them.

## Problem Statement / Motivation

The TUI currently has 15+ inline `Style` definitions scattered across 6 draw methods in a 3,329-line file. Only 4 named colors are used (`Cyan`, `DarkGray`, `Gray`, `Yellow`), all hardcoded. This makes it impossible to:

- Switch between dark and light appearances
- Let users customize the color scheme
- Maintain visual consistency (duplicate style expressions across methods)
- Ensure readability on light terminal backgrounds (e.g., `Color::Yellow` text is invisible on white)

## Proposed Solution

### Architecture

Create `src/tui/theme.rs` with:

1. **`Theme` struct** -- flat struct mapping 10 semantic UI roles to `ratatui::style::Style` values, plus an `is_dark: bool` field for identity
2. **Built-in constructors** -- `Theme::dark()` and `Theme::light()` using named terminal colors
3. **`toggle()` method** -- on `Theme` itself; returns the opposite theme. No separate enum needed.
4. **Runtime toggle** -- `T` (shift+t) in `Mode::Normal` calls `self.theme = self.theme.toggle()`; ratatui's immediate-mode rendering applies it on the next frame

### Semantic Role Mapping

Only roles that have a non-default, non-duplicate value today. Roles are added when the UI gains a visual distinction that requires them.

| Theme Field | Current Style | Used In |
|---|---|---|
| `active_border` | `fg(Cyan)` | Focused pane border, active tab text |
| `inactive_border` | `Style::default()` | Non-focused pane borders, inactive tab text |
| `inactive_tab` | `fg(DarkGray)` | Non-current screen tab text |
| `help_key` | `fg(Cyan) bg(DarkGray) BOLD` | Keyboard shortcut labels in help bar |
| `help_desc` | `fg(Gray)` | Shortcut descriptions in help bar |
| `highlight` | `REVERSED` | Cursor line (content, settings, library), tree selection |
| `visual_selection` | `bg(DarkGray)` | Selected text range in visual select mode |
| `input_border` | `fg(Yellow)` | Title input, rename input, edit mode borders |
| `edit_cursor_line` | `UNDERLINED` | Active line in tui-textarea editor |
| `active_tab` | `fg(Cyan) + BOLD` | Current screen tab text |

**Consolidated roles (reviewer feedback):**
- `cursor` + `tree_highlight` merged into `highlight` -- both are `REVERSED`
- `input_border` + `edit_border` merged into `input_border` -- both are `fg(Yellow)`
- `normal_text`, `status_bar`, `error_text`, `placeholder_text`, `scrollbar` removed -- all are `Style::default()` in both themes; add when they diverge

**Pre-implementation audit:** Before starting, run `rg "Style::default\(\)|fg\(|bg\(|Modifier::" src/tui/app.rs` to produce the definitive list and catch any missed sites.

### Key Decisions

- **No `ThemeMode` enum.** `Theme` stores `is_dark: bool`. Toggle swaps the struct directly: `self.theme = self.theme.toggle()`. One type, zero indirection.
- **Toggle key:** `T` (shift+t), active only in `Mode::Normal`. Avoids conflict with future vim-style lowercase bindings.
- **Module location:** `src/tui/theme.rs`, exported via `src/tui/mod.rs`.
- **Color strategy:** Named terminal colors (`Color::Cyan`, `Color::Blue`, etc.) for built-in themes. Respects user's terminal palette.
- **Theme storage:** `theme: Theme` stored directly on `App`. Draw methods access `self.theme`.
- **No auto-detection:** Always starts in dark mode.
- **No persistence:** Theme resets to dark on restart until #6 config support lands.
- **Derives:** `Theme` derives `Debug`, `Clone`, `PartialEq`.

## Technical Considerations

### ratatui Integration

- ratatui is immediate-mode: swapping `self.theme` takes effect on the very next `draw()` call with zero additional work
- `tui-textarea::TextArea` has its own styling API (`set_cursor_line_style`, `set_block`). The theme must be applied when creating the `TextArea` in `enter_edit_mode()`. Since `T` does not fire in edit mode, the textarea is always freshly created with the current theme.

### Light Theme Color Choices

The light theme must avoid colors that are invisible on white/light backgrounds:

| Role | Dark Theme | Light Theme | Rationale |
|---|---|---|---|
| Active border/tab | `Cyan` | `Blue` | Better contrast on white |
| Inactive elements | `DarkGray` | `Gray` | Slightly darker for visibility |
| Input border | `Yellow` | `Magenta` | Yellow on white is invisible |
| Visual selection bg | `DarkGray` | `LightYellow` | Visible highlight on light bg |
| Highlight | `REVERSED` | `REVERSED` | Modifier-only, works everywhere |

### Testing Strategy

- **Unit tests in `src/tui/theme.rs`:** Assert that `Theme::dark()` and `Theme::light()` return expected `Style` values for each field. Assert `toggle()` flips `is_dark` and returns the opposite palette. These are fast, isolated, and meaningful.
- **Existing tests pass unchanged:** Dark theme uses the exact same color values as current hardcoded styles, so all existing `TestBackend` assertions (e.g., `Modifier::REVERSED`) continue to pass.
- **No preemptive `TestBackend` cell-color tests.** Cell-level color assertions tie tests to exact buffer coordinates and break on any layout change. Add them only if a visual regression actually occurs.
- **Testable `App` construction:** `App::new()` takes `Vec<SourceRoot>`, which can be constructed with empty vecs in tests. No filesystem access required.

## Acceptance Criteria

- [x] New `src/tui/theme.rs` module with `Theme` struct (10 fields + `is_dark`)
- [x] `Theme::dark()` matches current visual appearance (backward compatible)
- [x] `Theme::light()` is readable on light terminal backgrounds
- [x] `Theme::toggle()` returns the opposite theme
- [x] All draw methods use `self.theme` -- zero hardcoded `Color::*` or `Style::default().fg(...)` remaining
- [x] `T` (shift+t) toggles theme in `Mode::Normal` on both screens
- [x] Help bar shows `T Theme` in Normal mode on both screens
- [x] All existing tests pass without modification

## Implementation Phases

All phases are sequential TDD cycles within a single PR. The value is delivered when `T` toggles between working themes.

### Phase 1: Theme Struct, Dark Palette, and Wire-up (TDD)

**Files:** `src/tui/theme.rs`, `src/tui/mod.rs`, `src/tui/app.rs`

1. Run `rg` audit to confirm the definitive list of styling sites
2. Write failing test: `Theme::dark().active_border` returns expected style
3. Create `Theme` struct with 10 semantic fields + `is_dark`, derive `Debug`, `Clone`, `PartialEq`
4. Implement `Theme::dark()` matching current hardcoded values exactly
5. Implement `Theme::toggle()` method
6. Add tests for all fields and toggle behavior
7. Add `theme: Theme` to `App`, initialize with `Theme::dark()` in `App::new()`
8. Replace inline styles in draw methods with `self.theme.<field>`, one method at a time:
   `draw_tab_bar()` -> `draw_files_screen()` borders -> `draw_content_pane()` -> `draw_settings_screen()` -> `draw_library_pane()` -> `help_line()` -> `draw()` input/status bars
9. After each replacement, verify existing tests still pass

### Phase 2: Light Theme and Toggle (TDD)

**Files:** `src/tui/theme.rs`, `src/tui/app.rs`

1. Write failing test: `Theme::light().active_border` differs from dark
2. Implement `Theme::light()` with light-appropriate colors
3. Write failing test: pressing `T` in Normal mode changes `self.theme.is_dark`
4. Add `T` handler in `handle_key_event()` at the same level as screen-switch keys (`1`/`2`), guarded by `Mode::Normal`
5. Add `T Theme` to help bar in Normal mode on both screens
6. Update `enter_edit_mode()` to use `self.theme.edit_cursor_line` and `self.theme.input_border`
7. Final `rg` pass: grep for any remaining `Color::` or `Style::default().fg` in draw methods
8. Clippy + full test pass

## Dependencies and Risks

**No new crate dependencies required.** The implementation uses only existing `ratatui::style` types.

**Risk: Large refactor surface.** Touching all 6 draw methods in a 3,329-line file. Mitigated by incremental TDD -- one style replacement per cycle.

**Risk: Visual regression.** Dark theme must look identical to current app. Mitigated by `Theme::dark()` using the exact same color values as current hardcoded styles.

**Deferred:** Custom TOML themes, terminal background auto-detection, theme persistence, catppuccin integration, `error_text`/`placeholder_text`/`scrollbar` theme roles -- all wait for #6 or until the UI gains non-default styles for them.

## References and Research

### Internal References

- Current styling: `src/tui/app.rs` -- 15+ inline `Style` definitions across lines 329-919
- Brainstorm: `docs/brainstorms/2026-02-27-context-manager-brainstorm.md` (line 86: themes listed as future)
- Issue #6: Configuration file support (open, deferred dependency)

### External References

- [gitui theme system](https://github.com/extrawurst/gitui/blob/master/THEMES.md) -- flat semantic-field struct pattern (adopted)
- [Yazi theme.toml](https://yazi-rs.github.io/docs/configuration/theme/) -- hierarchical nested struct pattern (rejected as overkill)
- [ratatui Style docs](https://docs.rs/ratatui/latest/ratatui/style/struct.Style.html)
- [ratatui serde feature](https://ratatui.rs/installation/feature-flags/) -- built-in Color/Style serialization (for future TOML support)
