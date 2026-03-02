use std::collections::HashSet;

use crate::library::Snippet;

/// Concatenates selected snippets in library order, separated by double newlines.
///
/// Iterates through the snippets slice, including only those whose index
/// appears in `selected`. Returns the concatenated content as a single string.
pub fn compose_snippets(snippets: &[Snippet], selected: &HashSet<usize>) -> String {
    snippets
        .iter()
        .enumerate()
        .filter(|(i, _)| selected.contains(i))
        .map(|(_, s)| s.content.as_str())
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
        let selected = HashSet::new();
        assert_eq!(compose_snippets(&snippets, &selected), "");
    }

    #[test]
    fn single_selection_returns_content_without_separator() {
        let snippets = vec![snippet("A", "aaa"), snippet("B", "bbb")];
        let selected = HashSet::from([1]);
        assert_eq!(compose_snippets(&snippets, &selected), "bbb");
    }

    #[test]
    fn multiple_selections_joined_with_double_newline() {
        let snippets = vec![
            snippet("A", "aaa"),
            snippet("B", "bbb"),
            snippet("C", "ccc"),
        ];
        let selected = HashSet::from([0, 2]);
        assert_eq!(compose_snippets(&snippets, &selected), "aaa\n\nccc");
    }

    #[test]
    fn preserves_library_order_not_selection_order() {
        let snippets = vec![
            snippet("First", "111"),
            snippet("Second", "222"),
            snippet("Third", "333"),
        ];
        // Select in reverse order — output should still be library order
        let selected = HashSet::from([2, 0]);
        assert_eq!(compose_snippets(&snippets, &selected), "111\n\n333");
    }

    #[test]
    fn all_selected() {
        let snippets = vec![snippet("A", "aaa"), snippet("B", "bbb")];
        let selected = HashSet::from([0, 1]);
        assert_eq!(compose_snippets(&snippets, &selected), "aaa\n\nbbb");
    }

    #[test]
    fn out_of_bounds_index_ignored() {
        let snippets = vec![snippet("A", "aaa")];
        let selected = HashSet::from([0, 5]);
        assert_eq!(compose_snippets(&snippets, &selected), "aaa");
    }

    #[test]
    fn empty_snippets_returns_empty() {
        let snippets: Vec<Snippet> = vec![];
        let selected = HashSet::from([0]);
        assert_eq!(compose_snippets(&snippets, &selected), "");
    }

    #[test]
    fn multiline_content_preserved() {
        let snippets = vec![
            snippet("A", "line 1\nline 2"),
            snippet("B", "line 3\nline 4"),
        ];
        let selected = HashSet::from([0, 1]);
        assert_eq!(
            compose_snippets(&snippets, &selected),
            "line 1\nline 2\n\nline 3\nline 4"
        );
    }
}
