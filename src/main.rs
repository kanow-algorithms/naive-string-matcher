use std::env::args as cli_args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let (file_path, patterns) = get_cli_arguments();
    let searched_file_content = load_whole_file_as_string(file_path.as_str());
    let longest_pattern_len = patterns
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len();
    let start = Instant::now();
    let matches_idx = naive_string_matcher(searched_file_content, patterns, longest_pattern_len);
    println!(
        "TIME IN NANOSECONDS: {}\nMATCHES: {:?}",
        start.elapsed().as_nanos(),
        matches_idx
    );
}

fn naive_string_matcher(
    searched_file_content: Vec<char>,
    patterns: Vec<String>,
    longest_pattern_len: usize,
) -> Vec<usize> {
    let mut idx = 0;
    let mut matched_index: Vec<usize> = vec![];
    let mut current_word: String = searched_file_content[0..longest_pattern_len]
        .to_vec()
        .iter()
        .collect();
    for ch in searched_file_content.into_iter().skip(longest_pattern_len) {
        patterns.iter().for_each(|pattern| {
            if current_word.starts_with(pattern) {
                matched_index.push(idx);
            }
        });
        current_word.drain(0..1);
        current_word.push(ch);
        idx += 1;
    }
    return matched_index;
}

fn get_cli_arguments() -> (String, Vec<String>) {
    let mut user_args: (String, Vec<String>) = (String::from(""), vec![]);
    let mut user_input: Vec<String> = cli_args().collect();
    user_input.remove(0);
    let mut user_input_iter = user_input.into_iter();
    user_args.0 = user_input_iter.next().unwrap();
    user_input_iter.for_each(|value| user_args.1.push(value));
    return user_args;
}

fn load_whole_file_as_string(file_path: &str) -> Vec<char> {
    let file = BufReader::new(File::open(file_path).unwrap());
    let mut file_as_string = String::from("");
    for line in file.lines() {
        match line {
            Ok(ln) => {
                file_as_string.push_str(ln.as_str());
            }
            _ => {}
        }
    }
    return file_as_string.chars().collect();
}
