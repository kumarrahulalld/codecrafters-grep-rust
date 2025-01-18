use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // If we have reached the end of the pattern
    if pind == pattern.len() {
        return ind == input_line.len(); // If we've reached the end of the pattern, check if input is exhausted
    }

    let pattern_char = pattern.chars().nth(pind).unwrap();

    // Handle escape sequences like \d, \w, etc.
    if pattern_char == '\\' {
        if pind + 1 < pattern.len() {
            let next_char = pattern.chars().nth(pind + 1).unwrap();
            match next_char {
                'd' => {
                    if ind < input_line.len() && input_line.chars().nth(ind).unwrap().is_digit(10) {
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    return false;
                }
                'w' => {
                    if ind < input_line.len() && input_line.chars().nth(ind).unwrap().is_alphanumeric() {
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    return false;
                }
                _ => return false,
            }
        } else {
            return false; // If escape is at the end of the pattern, return false
        }
    }

    // Handle ^ start of a line
    if pattern_char == '^' {
        let sub_str = &pattern[1..];
        return input_line.starts_with(sub_str);
    }

    // Handle $ end of a line
    if pattern_char == '$' {
        return ind == input_line.len(); // Ensure we are at the end of the input
    }

    // Handle ? (zero or one occurrence)
    if pattern_char == '?' {
        // Option 1: Skip the character before '?' (i.e., treat it as zero occurrence)
        let skip_match = match_pattern(input_line, pattern, ind, pind + 1);
        
        // Option 2: Match the previous character and continue (i.e., treat it as one occurrence)
        let match_current = ind < input_line.len() &&
                            input_line.chars().nth(ind).unwrap() == pattern.chars().nth(pind - 1).unwrap() &&
                            match_pattern(input_line, pattern, ind + 1, pind + 1);

        return skip_match || match_current;
    }

    // Handle character classes [abc] and [^abc]
    if pattern_char == '[' {
        let mut class_end = pind + 1;
        let mut is_negated = false;

        // Check if the class is negated (starts with [^)
        if pattern.chars().nth(pind + 1) == Some('^') {
            is_negated = true;
            class_end += 1;
        }

        // Find where the class ends
        while class_end < pattern.len() && pattern.chars().nth(class_end) != Some(']') {
            class_end += 1;
        }

        if class_end == pattern.len() {
            return false; // If we didn't find the closing ']', return false
        }

        let class_content = &pattern[pind + 1..class_end];
        let input_char = input_line.chars().nth(ind).unwrap();

        let class_match = if is_negated {
            !class_content.contains(input_char) // Negated class: matches if not in the class
        } else {
            class_content.contains(input_char) // Regular class: matches if in the class
        };

        if class_match {
            return match_pattern(input_line, pattern, ind + 1, class_end + 1); // Move past the class
        }
        return false; // If no match for the class
    }

    // Handle normal characters (not escape sequences or classes).
    if ind < input_line.len() && pattern_char == input_line.chars().nth(ind).unwrap() {
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Iterate through the input and try matching from each position
    for i in 0..=input_line.len() {
        if match_pattern(&input_line, &pattern, i, 0) {
            process::exit(0); // If a match is found, exit with success
        }
    }

    process::exit(1); // If no match is found, exit with failure
}
