use crate::naive_string_matcher;

#[test]
fn should_find_single_pattern_match_in_single_line_text() {
    let text = String::from("John is a nice guy");
    let pattern = String::from("nice");
    let matches = naive_string_matcher(&text.chars().collect(), &pattern.chars().collect());
    assert_eq!(matches[0], 10);
    assert_eq!(matches.len(), 1);
}

#[test]
fn should_find_three_matches_in_many_line_text() {
    let text = String::from("John is a nice guy\nTom is a nice guy\nBob is not a nice guy");
    let pattern = String::from("nice");
    let matches = naive_string_matcher(&text.chars().collect(), &pattern.chars().collect());
    assert_eq!(matches.len(), 3);
}

#[test]
fn should_find_match_in_the_end_of_text() {
    let text = vec!['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
    let pattern = vec!['w', 'o', 'r', 'l', 'd'];
    let matches = naive_string_matcher(&text, &pattern);
    assert_eq!(matches[0], 6);
}

#[test]
fn should_not_find_any_match() {
    let text = String::from("John is a nice guy\nTom is a nice guy\nBob is not a nice guy");
    let pattern = String::from("Kris");
    let matches = naive_string_matcher(&text.chars().collect(), &pattern.chars().collect());
    assert!(matches.is_empty());
}
