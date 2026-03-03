use crate::library::Snippet;

/// Concatenates selected snippets in selection order, separated by double newlines.
///
/// The `selected` slice contains snippet indices in the order they were selected.
/// Out-of-bounds indices are silently skipped.
pub fn compose_snippets(snippets: &[Snippet], selected: &[usize]) -> String {
    selected
        .iter()
        .filter_map(|&i| snippets.get(i))
        .map(|s| s.content.as_str())
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snippet(title: &str, content: &str) -> Snippet {
        Snippet {
            title: title.to_string(),
            content: content.to_string(),
            source: String::new(),
        }
    }

    #[test]
    fn empty_selection_returns_empty_string() {
        let snippets = vec![snippet("A", "aaa"), snippet("B", "bbb")];
        assert_eq!(compose_snippets(&snippets, &[]), "");
    }

    #[test]
    fn single_selection_returns_content_without_separator() {
        let snippets = vec![snippet("A", "aaa"), snippet("B", "bbb")];
        assert_eq!(compose_snippets(&snippets, &[1]), "bbb");
    }

    #[test]
    fn multiple_selections_joined_with_double_newline() {
        let snippets = vec![
            snippet("A", "aaa"),
            snippet("B", "bbb"),
            snippet("C", "ccc"),
        ];
        assert_eq!(compose_snippets(&snippets, &[0, 2]), "aaa\n\nccc");
    }

    #[test]
    fn preserves_selection_order() {
        let snippets = vec![
            snippet("First", "111"),
            snippet("Second", "222"),
            snippet("Third", "333"),
        ];
        // Select third then first — output follows selection order
        assert_eq!(compose_snippets(&snippets, &[2, 0]), "333\n\n111");
    }

    #[test]
    fn all_selected() {
        let snippets = vec![snippet("A", "aaa"), snippet("B", "bbb")];
        assert_eq!(compose_snippets(&snippets, &[0, 1]), "aaa\n\nbbb");
    }

    #[test]
    fn out_of_bounds_index_ignored() {
        let snippets = vec![snippet("A", "aaa")];
        assert_eq!(compose_snippets(&snippets, &[0, 5]), "aaa");
    }

    #[test]
    fn empty_snippets_returns_empty() {
        let snippets: Vec<Snippet> = vec![];
        assert_eq!(compose_snippets(&snippets, &[0]), "");
    }

    #[test]
    fn multiline_content_preserved() {
        let snippets = vec![
            snippet("A", "line 1\nline 2"),
            snippet("B", "line 3\nline 4"),
        ];
        assert_eq!(
            compose_snippets(&snippets, &[0, 1]),
            "line 1\nline 2\n\nline 3\nline 4"
        );
    }
}
