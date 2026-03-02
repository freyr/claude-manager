/// Color theme definitions for the TUI.
///
/// Maps semantic UI roles to `ratatui::style::Style` values. Provides
/// built-in dark and light palettes and a `toggle()` method to swap between
/// them at runtime.
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;

/// Maps every visual role in the application to a concrete [`Style`].
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// Whether this is the dark variant.
    pub is_dark: bool,
    /// Focused pane border and active tab text.
    pub active_border: Style,
    /// Non-focused pane borders.
    pub inactive_border: Style,
    /// Active screen tab text.
    pub active_tab: Style,
    /// Inactive screen tab text.
    pub inactive_tab: Style,
    /// Keyboard shortcut labels in the help bar.
    pub help_key: Style,
    /// Shortcut descriptions in the help bar.
    pub help_desc: Style,
    /// Cursor line and selected tree/list item.
    pub highlight: Style,
    /// Selected text range in visual select mode.
    pub visual_selection: Style,
    /// Border for input fields and edit mode.
    pub input_border: Style,
    /// Active line in the text editor.
    pub edit_cursor_line: Style,
}

impl Theme {
    /// Returns the built-in dark palette matching the original hardcoded styles.
    pub fn dark() -> Self {
        Self {
            is_dark: true,
            active_border: Style::default().fg(Color::Cyan),
            inactive_border: Style::default(),
            active_tab: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            inactive_tab: Style::default().fg(Color::DarkGray),
            help_key: Style::default()
                .fg(Color::Cyan)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
            help_desc: Style::default().fg(Color::Gray),
            highlight: Style::default().add_modifier(Modifier::REVERSED),
            visual_selection: Style::default().bg(Color::DarkGray),
            input_border: Style::default().fg(Color::Yellow),
            edit_cursor_line: Style::default().add_modifier(Modifier::UNDERLINED),
        }
    }

    /// Returns the built-in light palette with colors readable on light
    /// terminal backgrounds.
    pub fn light() -> Self {
        Self {
            is_dark: false,
            active_border: Style::default().fg(Color::Blue),
            inactive_border: Style::default().fg(Color::Gray),
            active_tab: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            inactive_tab: Style::default().fg(Color::Gray),
            help_key: Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            help_desc: Style::default().fg(Color::DarkGray),
            highlight: Style::default().add_modifier(Modifier::REVERSED),
            visual_selection: Style::default().bg(Color::LightYellow),
            input_border: Style::default().fg(Color::Magenta),
            edit_cursor_line: Style::default().add_modifier(Modifier::UNDERLINED),
        }
    }

    /// Returns the opposite theme (dark ↔ light).
    pub fn toggle(&self) -> Self {
        if self.is_dark {
            Self::light()
        } else {
            Self::dark()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dark_theme_active_border_is_cyan() {
        let theme = Theme::dark();
        assert_eq!(theme.active_border, Style::default().fg(Color::Cyan));
    }

    #[test]
    fn dark_theme_is_dark_flag() {
        assert!(Theme::dark().is_dark);
    }

    #[test]
    fn light_theme_is_not_dark() {
        assert!(!Theme::light().is_dark);
    }

    #[test]
    fn dark_theme_matches_original_hardcoded_values() {
        let t = Theme::dark();
        assert_eq!(t.active_border, Style::default().fg(Color::Cyan));
        assert_eq!(t.inactive_border, Style::default());
        assert_eq!(
            t.active_tab,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        );
        assert_eq!(t.inactive_tab, Style::default().fg(Color::DarkGray));
        assert_eq!(
            t.help_key,
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        );
        assert_eq!(t.help_desc, Style::default().fg(Color::Gray));
        assert_eq!(
            t.highlight,
            Style::default().add_modifier(Modifier::REVERSED)
        );
        assert_eq!(t.visual_selection, Style::default().bg(Color::DarkGray));
        assert_eq!(t.input_border, Style::default().fg(Color::Yellow));
        assert_eq!(
            t.edit_cursor_line,
            Style::default().add_modifier(Modifier::UNDERLINED)
        );
    }

    #[test]
    fn light_theme_differs_from_dark() {
        let dark = Theme::dark();
        let light = Theme::light();
        assert_ne!(dark.active_border, light.active_border);
        assert_ne!(dark.input_border, light.input_border);
        assert_ne!(dark.visual_selection, light.visual_selection);
    }

    #[test]
    fn toggle_dark_returns_light() {
        let dark = Theme::dark();
        let toggled = dark.toggle();
        assert!(!toggled.is_dark);
        assert_eq!(toggled, Theme::light());
    }

    #[test]
    fn toggle_light_returns_dark() {
        let light = Theme::light();
        let toggled = light.toggle();
        assert!(toggled.is_dark);
        assert_eq!(toggled, Theme::dark());
    }

    #[test]
    fn double_toggle_returns_original() {
        let original = Theme::dark();
        let round_trip = original.toggle().toggle();
        assert_eq!(original, round_trip);
    }
}
