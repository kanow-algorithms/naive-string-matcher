fn naive_string_matcher(text: &Vec<char>, pattern: &Vec<char>) -> Vec<usize> {
    let mut matches: Vec<usize> = vec![];
    let pattern_length = pattern.len();
    let stop_length = text.len() - pattern_length;
    for space_index in 0..stop_length {
        let mut does_match = true;
        let mut searched_frame_index = space_index;
        for pattern_letter in pattern {
            if *pattern_letter != text[searched_frame_index] {
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

#[cfg(test)]
mod tests;
