pub fn naive_string_matcher(text: &Vec<char>, pattern: &Vec<char>) -> Vec<usize> {
    let mut matches: Vec<usize> = vec![];
    let stop_index = text.len() - pattern.len();
    for current_index in 0..stop_index {
        let mut does_match = true;
        let mut searched_frame_index = current_index;
        for pattern_letter in pattern {
            if *pattern_letter != text[searched_frame_index] {
                does_match = false;
                break;
            }
            searched_frame_index += 1;
        }
        if does_match {
            matches.push(current_index)
        }
    }
    return matches;
}

#[cfg(test)]
mod tests;
