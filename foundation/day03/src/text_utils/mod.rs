use crate::domain::is_empty_like;

pub fn normalize_whitespace(input: &str) -> String {
    let trimmed = input.trim();
    let mut result = String::new();
    let mut prev_was_space = false;

    for ch in trimmed.chars() {
        if ch.is_whitespace() {
            if !prev_was_space {
                result.push(' ');
                prev_was_space = true;
            }
        } else {
            result.push(ch);
            prev_was_space = false;
        }
    }

    result
}

pub fn count_words(input: &str) -> usize {
    let norm = normalize_whitespace(input);
    if is_empty_like(&norm) {
        0
    } else {
        norm.split_whitespace().count()
    }
}

pub fn summarize(input: &str) -> (usize, usize, bool) {
    let norm = normalize_whitespace(input);
    let empty = is_empty_like(&norm);
    let words = count_words(&norm);
    let chars = norm.chars().count();

    (chars, words, empty)
}

pub fn make_preview(input: &str, max_len: usize) -> String {
    let norm = normalize_whitespace(&input);
    let count = norm.chars().count();

    if count == 0 {
        return norm;
    }

    if max_len == 0 {
        return String::from("...");
    }

    if count > max_len {
        let mut preview = norm.chars().take(max_len).collect::<String>();
        preview.push_str("...");
        preview
    } else {
        norm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_newlines_collapsed() {
        assert_eq!(normalize_whitespace("  hello   world  "), "hello world");
    }

    #[test]
    fn tabs_and_newlines_collapsed() {
        assert_eq!(normalize_whitespace("a\t\tb\n\nc"), "a b c");
    }

    #[test]
    fn empty_is_zero() {
        assert_eq!(count_words("     "), 0);
    }

    #[test]
    fn simple_sentence() {
        assert_eq!(count_words("hello world"), 2);
    }

    #[test]
    fn mixed_spaces() {
        assert_eq!(count_words("  a\t b \n c  "), 3);
    }

    #[test]
    fn empty_like_true() {
        assert_eq!(summarize("  "), (0, 0, true));
    }

    #[test]
    fn simple_line() {
        assert_eq!(summarize("hello world"), (11, 2, false))
    }

    #[test]
    fn longer_than_limit_with_ellipsis() {
        assert_eq!(make_preview("hello world", 5), "hello...");
    }

    #[test]
    fn exactly_limit_no_ellipsis() {
        assert_eq!(make_preview("hello", 5), "hello");
    }

    #[test]
    fn shorter_than_limit_no_ellipsis() {
        assert_eq!(make_preview("hi", 5), "hi");
    }

    #[test]
    fn zero_limit_edge_case() {
        assert_eq!(make_preview("anything", 0), "...");
    }

    #[test]
    fn empty_string_stays_empty() {
        assert_eq!(make_preview("   ", 5), "");
    }
}
