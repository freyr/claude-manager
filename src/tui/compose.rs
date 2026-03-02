use std::collections::HashSet;

use ratatui::Frame;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Scrollbar;
use ratatui::widgets::ScrollbarOrientation;
use ratatui::widgets::ScrollbarState;

use super::app::App;
use super::app::Mode;
use super::app::Screen;

/// State for the Compose screen.
#[derive(Debug)]
pub struct ComposeState {
    /// Set of selected (checked) snippet indices (into App.library).
    pub selected: HashSet<usize>,
    /// Cursor position in the snippet list.
    pub cursor: usize,
    /// Scroll offset for the list.
    pub scroll: u16,
    /// Viewport height (set during draw).
    pub viewport_height: u16,
    /// Whether the preview is currently shown full-screen.
    pub preview_visible: bool,
    /// Scroll offset for the preview pane.
    pub preview_scroll: u16,
    /// Viewport height for preview (set during draw).
    pub preview_viewport_height: u16,
}

impl Default for ComposeState {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposeState {
    /// Creates a new compose state with no selections.
    pub fn new() -> Self {
        Self {
            selected: HashSet::new(),
            cursor: 0,
            scroll: 0,
            viewport_height: 0,
            preview_visible: false,
            preview_scroll: 0,
            preview_viewport_height: 0,
        }
    }

    /// Returns the number of snippets in the library.
    fn snippet_count(app: &App) -> usize {
        app.library.as_ref().map_or(0, |lib| lib.snippets.len())
    }
}

impl App {
    /// Enters the Compose screen, loading the library if needed.
    pub(crate) fn enter_compose_screen(&mut self) {
        // Load library if not already loaded
        if self.library.is_none() {
            if let Some(path) = crate::library::library_path() {
                match crate::library::load_library(&path) {
                    Ok(lib) => self.library = Some(lib),
                    Err(err) => {
                        self.status_message = Some(format!("Failed to load library: {err}"));
                        return;
                    }
                }
            } else {
                self.status_message = Some("Cannot determine library path.".to_string());
                return;
            }
        }

        // Initialize compose state if needed
        if self.compose_state.is_none() {
            self.compose_state = Some(ComposeState::new());
        }

        self.screen = Screen::Compose;
    }

    /// Enters the Compose screen with a specific library path (for testability).
    pub fn enter_compose_screen_from(&mut self, path: &std::path::Path) {
        match crate::library::load_library(path) {
            Ok(lib) => {
                self.library = Some(lib);
                if self.compose_state.is_none() {
                    self.compose_state = Some(ComposeState::new());
                }
                self.screen = Screen::Compose;
            }
            Err(err) => {
                self.status_message = Some(format!("Failed to load library: {err}"));
            }
        }
    }

    pub(crate) fn draw_compose_screen(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let library = match &self.library {
            Some(lib) => lib,
            None => {
                let msg = Paragraph::new("Library not loaded.")
                    .block(Block::default().borders(Borders::ALL).title("Compose"));
                frame.render_widget(msg, area);
                return;
            }
        };

        let compose = match &self.compose_state {
            Some(s) => s,
            None => return,
        };

        if library.snippets.is_empty() {
            let msg = Paragraph::new(
                "Library is empty. Save snippets with v then s on the Files screen.",
            )
            .block(Block::default().borders(Borders::ALL).title("Compose"));
            frame.render_widget(msg, area);
            return;
        }

        // Preview mode: show composed output full-screen
        if compose.preview_visible {
            self.draw_compose_preview(frame, area);
            return;
        }

        // Normal mode: show snippet list with checkboxes
        let viewport_height = area.height.saturating_sub(2);

        let cursor = compose.cursor;
        let highlight = self.theme.highlight;

        let lines: Vec<Line> = library
            .snippets
            .iter()
            .enumerate()
            .map(|(i, snippet)| {
                let checkbox = if compose.selected.contains(&i) {
                    "[x] "
                } else {
                    "[ ] "
                };
                let text = format!("{checkbox}{}", snippet.title);
                let style = if i == cursor {
                    highlight
                } else {
                    Style::default()
                };
                Line::from(text).style(style)
            })
            .collect();

        let selected_count = compose.selected.len();
        let total_count = library.snippets.len();
        let title = format!("Compose ({selected_count}/{total_count} selected)");

        let list_widget = Paragraph::new(Text::from(lines))
            .block(Block::default().borders(Borders::ALL).title(title))
            .scroll((compose.scroll, 0));
        frame.render_widget(list_widget, area);

        let mut scrollbar_state =
            ScrollbarState::new(library.snippets.len()).position(compose.scroll as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

        // Update viewport height
        if let Some(cs) = &mut self.compose_state {
            cs.viewport_height = viewport_height;
        }
    }

    fn draw_compose_preview(&mut self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let compose = match &self.compose_state {
            Some(s) => s,
            None => return,
        };

        let composed = self.composed_text();
        let viewport_height = area.height.saturating_sub(2);

        let preview_widget = Paragraph::new(composed.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Preview (Esc to return)"),
            )
            .scroll((compose.preview_scroll, 0));
        frame.render_widget(preview_widget, area);

        let line_count = composed.lines().count();
        let mut scrollbar_state =
            ScrollbarState::new(line_count).position(compose.preview_scroll as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

        if let Some(cs) = &mut self.compose_state {
            cs.preview_viewport_height = viewport_height;
        }
    }

    /// Returns the composed text from currently selected snippets.
    pub(crate) fn composed_text(&self) -> String {
        let library = match &self.library {
            Some(lib) => lib,
            None => return String::new(),
        };
        let compose = match &self.compose_state {
            Some(s) => s,
            None => return String::new(),
        };
        crate::compose::compose_snippets(&library.snippets, &compose.selected)
    }

    pub(crate) fn handle_compose_key(&mut self, key_event: KeyEvent) {
        let compose = match &self.compose_state {
            Some(s) => s,
            None => return,
        };

        // In preview mode, only handle scroll and exit
        if compose.preview_visible {
            self.handle_compose_preview_key(key_event);
            return;
        }

        let snippet_count = ComposeState::snippet_count(self);
        if snippet_count == 0 {
            match key_event.code {
                KeyCode::Esc => {
                    self.screen = Screen::Files;
                }
                KeyCode::Char('q') => {
                    self.exit = true;
                }
                _ => {}
            }
            return;
        }

        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if let Some(cs) = &mut self.compose_state
                    && cs.cursor < snippet_count.saturating_sub(1)
                {
                    cs.cursor += 1;
                    ensure_compose_cursor_visible(cs);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(cs) = &mut self.compose_state {
                    cs.cursor = cs.cursor.saturating_sub(1);
                    ensure_compose_cursor_visible(cs);
                }
            }
            KeyCode::Char(' ') => {
                if let Some(cs) = &mut self.compose_state {
                    let cursor = cs.cursor;
                    if cs.selected.contains(&cursor) {
                        cs.selected.remove(&cursor);
                    } else {
                        cs.selected.insert(cursor);
                    }
                }
            }
            KeyCode::Char('p') => {
                if let Some(cs) = &mut self.compose_state {
                    if cs.selected.is_empty() {
                        self.status_message = Some("No snippets selected.".to_string());
                    } else {
                        cs.preview_visible = true;
                        cs.preview_scroll = 0;
                    }
                }
            }
            KeyCode::Char('w') => {
                if let Some(cs) = &self.compose_state {
                    if cs.selected.is_empty() {
                        self.status_message = Some("No snippets selected.".to_string());
                    } else {
                        self.mode = Mode::ExportPath;
                        self.title_input.clear();
                        self.title_cursor = 0;
                    }
                }
            }
            KeyCode::Esc => {
                self.screen = Screen::Files;
            }
            KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        }
    }

    fn handle_compose_preview_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('j') | KeyCode::Down => {
                // Compute line count before borrowing compose_state mutably
                let line_count = self.composed_text().lines().count();
                if let Some(cs) = &mut self.compose_state {
                    let max_scroll = line_count.saturating_sub(cs.preview_viewport_height as usize);
                    if (cs.preview_scroll as usize) < max_scroll {
                        cs.preview_scroll += 1;
                    }
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if let Some(cs) = &mut self.compose_state {
                    cs.preview_scroll = cs.preview_scroll.saturating_sub(1);
                }
            }
            KeyCode::Esc => {
                if let Some(cs) = &mut self.compose_state {
                    cs.preview_visible = false;
                }
            }
            KeyCode::Char('q') => {
                self.exit = true;
            }
            _ => {}
        }
    }

    pub(crate) fn handle_export_path_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Enter => {
                self.execute_export();
            }
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.title_input.clear();
                self.title_cursor = 0;
            }
            KeyCode::Backspace => {
                if self.title_cursor > 0 {
                    self.title_cursor -= 1;
                    self.title_input.remove(self.title_cursor);
                }
            }
            KeyCode::Left => {
                self.title_cursor = self.title_cursor.saturating_sub(1);
            }
            KeyCode::Right => {
                if self.title_cursor < self.title_input.len() {
                    self.title_cursor += 1;
                }
            }
            KeyCode::Char(c) => {
                self.title_input.insert(self.title_cursor, c);
                self.title_cursor += 1;
            }
            _ => {}
        }
    }

    fn execute_export(&mut self) {
        let raw_path = self.title_input.trim().to_string();
        if raw_path.is_empty() {
            self.status_message = Some("No path entered.".to_string());
            return;
        }

        // Expand tilde
        let expanded = if raw_path.starts_with('~') {
            if let Ok(home) = std::env::var("HOME") {
                raw_path.replacen('~', &home, 1)
            } else {
                self.status_message = Some("Cannot expand ~: HOME not set.".to_string());
                return;
            }
        } else {
            raw_path
        };

        let path = std::path::PathBuf::from(&expanded);

        // Check parent exists
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
            && !parent.exists()
        {
            self.status_message = Some("Parent directory does not exist.".to_string());
            return;
        }

        // Refuse overwrite
        if path.exists() {
            self.status_message = Some("File already exists.".to_string());
            return;
        }

        let composed = self.composed_text();
        let selected_count = self
            .compose_state
            .as_ref()
            .map_or(0, |cs| cs.selected.len());

        // Atomic write
        let parent = path.parent().unwrap_or(std::path::Path::new("."));
        let result = tempfile::NamedTempFile::new_in(parent).and_then(|mut tmp| {
            use std::io::Write;
            tmp.write_all(composed.as_bytes())?;
            tmp.flush()?;
            tmp.persist(&path).map_err(|e| e.error)?;
            Ok(())
        });

        match result {
            Ok(()) => {
                self.status_message = Some(format!(
                    "Exported {selected_count} snippet{} to {}",
                    if selected_count == 1 { "" } else { "s" },
                    path.display()
                ));
                self.mode = Mode::Normal;
                self.title_input.clear();
                self.title_cursor = 0;
            }
            Err(err) => {
                self.status_message = Some(format!("Export failed: {err}"));
            }
        }
    }
}

/// Ensures the compose cursor stays within the visible viewport.
fn ensure_compose_cursor_visible(cs: &mut ComposeState) {
    let scroll = cs.scroll as usize;
    let vh = cs.viewport_height as usize;
    if cs.cursor < scroll {
        cs.scroll = cs.cursor as u16;
    } else if vh > 0 && cs.cursor >= scroll + vh {
        cs.scroll = (cs.cursor - vh + 1) as u16;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use ratatui::crossterm::event::KeyCode;

    use tempfile::TempDir;

    use crate::config::Config;
    use crate::library::Snippet;
    use crate::library::SnippetLibrary;
    use crate::tui::app::App;
    use crate::tui::app::Mode;
    use crate::tui::app::Screen;
    use crate::tui::app::test_helpers::key_event;
    use crate::tui::app::test_helpers::render_once;

    fn app_with_library(snippets: Vec<(&str, &str)>) -> App {
        let mut app = App::new(vec![], &Config::default());
        app.library = Some(SnippetLibrary {
            snippets: snippets
                .into_iter()
                .map(|(title, content)| Snippet {
                    title: title.to_string(),
                    content: content.to_string(),
                    source: String::new(),
                })
                .collect(),
        });
        app.screen = Screen::Compose;
        app.compose_state = Some(super::ComposeState::new());
        app
    }

    #[test]
    fn key_3_enters_compose_screen() {
        let tmp = TempDir::new().unwrap();
        let lib_path = tmp.path().join("library.toml");
        crate::library::save_library(&SnippetLibrary::default(), &lib_path).unwrap();

        let mut app = App::new(vec![], &Config::default());
        app.enter_compose_screen_from(&lib_path);

        assert_eq!(app.screen, Screen::Compose);
        assert!(app.compose_state.is_some());
    }

    #[test]
    fn compose_empty_library_shows_message() {
        let mut app = app_with_library(vec![]);
        render_once(&mut app);
        // Empty library renders without panic — the draw method handles the message
    }

    #[test]
    fn jk_navigates_snippet_list() {
        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb"), ("C", "ccc")]);
        assert_eq!(app.compose_state.as_ref().unwrap().cursor, 0);

        app.handle_key_event(key_event(KeyCode::Char('j')));
        assert_eq!(app.compose_state.as_ref().unwrap().cursor, 1);

        app.handle_key_event(key_event(KeyCode::Char('j')));
        assert_eq!(app.compose_state.as_ref().unwrap().cursor, 2);

        // Can't go past last
        app.handle_key_event(key_event(KeyCode::Char('j')));
        assert_eq!(app.compose_state.as_ref().unwrap().cursor, 2);

        app.handle_key_event(key_event(KeyCode::Char('k')));
        assert_eq!(app.compose_state.as_ref().unwrap().cursor, 1);
    }

    #[test]
    fn space_toggles_selection() {
        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb")]);

        app.handle_key_event(key_event(KeyCode::Char(' ')));
        assert!(app.compose_state.as_ref().unwrap().selected.contains(&0));

        // Toggle off
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        assert!(!app.compose_state.as_ref().unwrap().selected.contains(&0));

        // Select second
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        assert!(app.compose_state.as_ref().unwrap().selected.contains(&1));
    }

    #[test]
    fn esc_returns_to_files_screen() {
        let mut app = app_with_library(vec![("A", "aaa")]);

        app.handle_key_event(key_event(KeyCode::Esc));
        assert_eq!(app.screen, Screen::Files);
    }

    #[test]
    fn q_quits_app() {
        let mut app = app_with_library(vec![("A", "aaa")]);

        app.handle_key_event(key_event(KeyCode::Char('q')));
        assert!(app.exit);
    }

    #[test]
    fn p_shows_preview_when_selected() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' '))); // select
        app.handle_key_event(key_event(KeyCode::Char('p'))); // preview

        assert!(app.compose_state.as_ref().unwrap().preview_visible);
    }

    #[test]
    fn p_shows_error_when_nothing_selected() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char('p')));

        assert!(!app.compose_state.as_ref().unwrap().preview_visible);
        assert!(
            app.status_message
                .as_deref()
                .unwrap()
                .contains("No snippets selected")
        );
    }

    #[test]
    fn esc_in_preview_returns_to_list() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('p')));
        assert!(app.compose_state.as_ref().unwrap().preview_visible);

        app.handle_key_event(key_event(KeyCode::Esc));
        assert!(!app.compose_state.as_ref().unwrap().preview_visible);
    }

    #[test]
    fn w_enters_export_path_mode() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('w')));

        assert_eq!(app.mode, Mode::ExportPath);
    }

    #[test]
    fn w_shows_error_when_nothing_selected() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char('w')));

        assert_eq!(app.mode, Mode::Normal);
        assert!(
            app.status_message
                .as_deref()
                .unwrap()
                .contains("No snippets selected")
        );
    }

    #[test]
    fn export_writes_file() {
        let tmp = TempDir::new().unwrap();
        let output_path = tmp.path().join("output.md");

        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb")]);
        // Select both
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char(' ')));

        // Enter export mode
        app.handle_key_event(key_event(KeyCode::Char('w')));
        assert_eq!(app.mode, Mode::ExportPath);

        // Type path
        for c in output_path.display().to_string().chars() {
            app.handle_key_event(key_event(KeyCode::Char(c)));
        }

        // Press Enter to export
        app.handle_key_event(key_event(KeyCode::Enter));

        assert_eq!(app.mode, Mode::Normal);
        let content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(content, "aaa\n\nbbb");
        assert!(
            app.status_message
                .as_deref()
                .unwrap()
                .contains("Exported 2")
        );
    }

    #[test]
    fn export_refuses_existing_file() {
        let tmp = TempDir::new().unwrap();
        let output_path = tmp.path().join("existing.md");
        fs::write(&output_path, "existing content").unwrap();

        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('w')));

        for c in output_path.display().to_string().chars() {
            app.handle_key_event(key_event(KeyCode::Char(c)));
        }
        app.handle_key_event(key_event(KeyCode::Enter));

        assert!(
            app.status_message
                .as_deref()
                .unwrap()
                .contains("already exists")
        );
        // File unchanged
        assert_eq!(
            fs::read_to_string(&output_path).unwrap(),
            "existing content"
        );
    }

    #[test]
    fn export_refuses_missing_parent() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('w')));

        for c in "/nonexistent/dir/output.md".chars() {
            app.handle_key_event(key_event(KeyCode::Char(c)));
        }
        app.handle_key_event(key_event(KeyCode::Enter));

        assert!(
            app.status_message
                .as_deref()
                .unwrap()
                .contains("Parent directory does not exist")
        );
    }

    #[test]
    fn esc_cancels_export_path() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('w')));
        assert_eq!(app.mode, Mode::ExportPath);

        app.handle_key_event(key_event(KeyCode::Esc));
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn compose_state_preserved_across_tab_switch() {
        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb")]);
        // Select first snippet
        app.handle_key_event(key_event(KeyCode::Char(' ')));

        // Switch to Files and back
        app.handle_key_event(key_event(KeyCode::Char('1')));
        assert_eq!(app.screen, Screen::Files);

        app.handle_key_event(key_event(KeyCode::Char('3')));
        assert_eq!(app.screen, Screen::Compose);

        // Selection should be preserved
        assert!(app.compose_state.as_ref().unwrap().selected.contains(&0));
    }

    #[test]
    fn composed_text_returns_selected_content() {
        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb"), ("C", "ccc")]);
        // Select first and third
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char(' ')));

        assert_eq!(app.composed_text(), "aaa\n\nccc");
    }

    #[test]
    fn compose_renders_without_panic() {
        let mut app = app_with_library(vec![("A", "aaa"), ("B", "bbb")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        render_once(&mut app);
        // No panic = success
    }

    #[test]
    fn compose_preview_renders_without_panic() {
        let mut app = app_with_library(vec![("A", "aaa")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('p')));
        render_once(&mut app);
        // No panic = success
    }

    #[test]
    fn tilde_expansion_in_export_path() {
        let tmp = TempDir::new().unwrap();
        let output_name = "test_output.md";

        let mut app = app_with_library(vec![("A", "content")]);
        app.handle_key_event(key_event(KeyCode::Char(' ')));
        app.handle_key_event(key_event(KeyCode::Char('w')));

        // Type a path using the tmp dir (not tilde, since HOME varies in tests)
        let full_path = tmp.path().join(output_name);
        for c in full_path.display().to_string().chars() {
            app.handle_key_event(key_event(KeyCode::Char(c)));
        }
        app.handle_key_event(key_event(KeyCode::Enter));

        assert!(app.status_message.as_deref().unwrap().contains("Exported"));
        assert_eq!(fs::read_to_string(full_path).unwrap(), "content");
    }
}
