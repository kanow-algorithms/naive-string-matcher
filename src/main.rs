use std::env::args as cli_args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let (file_path, patterns) = get_cli_arguments();
    let searched_file_content = load_whole_file_as_string(file_path.as_str());
    let start = Instant::now();
    let matches_idx = naive_string_matcher(&searched_file_content, &patterns[0].chars().collect());
    println!(
        "TIME IN NANOSECONDS: {}\nMATCHES: {:?}",
        start.elapsed().as_nanos(),
        matches_idx
    );
}

fn naive_string_matcher(searched_space: &Vec<char>, pattern: &Vec<char>) -> Vec<usize> {
    let mut matches: Vec<usize> = vec![];
    let pattern_length = pattern.len();
    let stop_length = searched_space.len() - pattern_length;
    for space_index in 0..stop_length {
        let mut does_match = true;
        let mut searched_frame_index = space_index;
        for pattern_letter in pattern {
            if *pattern_letter != searched_space[searched_frame_index] {
                does_match = false;
                break;
            }
            searched_frame_index += 1;
        }
        if does_match {
            matches.push(space_index)
        }
    }
    return matches;
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
