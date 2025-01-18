use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // If we have reached the end of the pattern
    if pind == pattern.len() {
        println!("Base case reached, pattern fully matched.");
        return true;
    }

    let pattern_char = pattern.chars().nth(pind).unwrap();

    // Handle escape sequences like \d, \w, etc.
    if pattern_char == '\\' {
        if pind + 1 < pattern.len() {
            let next_char = pattern.chars().nth(pind + 1).unwrap();
            println!("Escape sequence '\\{}' found", next_char);
            match next_char {
                'd' => {
                    // If current input is a digit, match the next part of the pattern
                    if ind < input_line.len() && input_line.chars().nth(ind).unwrap().is_digit(10) {
                        println!("Matched '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap());
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    println!("Failed to match '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '));
                    return false;
                }
                'w' => {
                    // If current input is alphanumeric, match the next part of the pattern
                    if ind < input_line.len() && input_line.chars().nth(ind).unwrap().is_alphanumeric() {
                        println!("Matched '\\w' (alphanumeric) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap());
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    println!("Failed to match '\\w' (alphanumeric) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '));
                    return false;
                }
                _ => {
                    println!("Unsupported escape sequence '\\{}' at pattern[{}]", next_char, pind);
                    return false;
                }
            }
        } else {
            println!("Escape sequence '\\' is at the end of the pattern, returning false");
            return false;
        }
    }

    // Handle ^ start of a line
    if pattern_char == '^' {
        let sub_str = &pattern[1..];
        println!("Substr {:?}", sub_str);
        return input_line.starts_with(sub_str);
    }

    // Handle $ end of a line
    if pattern_char == '$' {
        return ind == input_line.len();
    }

    // Handle + (one or more)
    if pattern_char == '+' {
        return ind < input_line.len();
    }

    // Handle ? (zero or one)
    if pattern_char == '?' {
        let skip_match = match_pattern(input_line, pattern, ind, pind + 1);
        let match_current = ind < input_line.len() &&
                            input_line.chars().nth(ind).unwrap() == pattern.chars().nth(pind - 1).unwrap() &&
                            match_pattern(input_line, pattern, ind + 1, pind + 1);

        return skip_match || match_current;
    }

    // Handle character classes [abc] and [^abc]
    if pattern_char == '[' {
        let mut class_end = pind + 1;
        let mut is_negated = false;

        if pattern.chars().nth(pind + 1) == Some('^') {
            is_negated = true;
            class_end += 1;
            println!("Negated class '[^...]' detected");
        }

        while class_end < pattern.len() && pattern.chars().nth(class_end) != Some(']') {
            class_end += 1;
        }

        if class_end == pattern.len() {
            println!("Failed to find closing ']' for class, returning false");
            return false;
        }

        let class_content = &pattern[pind + 1..class_end];
        let input_char = input_line.chars().nth(ind).unwrap();

        println!("Matching input[{}]: '{}' against class '{}'", ind, input_char, class_content);

        let class_match = if is_negated {
            !class_content.contains(input_char)
        } else {
            class_content.contains(input_char)
        };

        if class_match {
            println!("Class match successful for input[{}]: '{}'", ind, input_char);
            return match_pattern(input_line, pattern, ind + 1, class_end + 1); // Move past the class
        }
        return false;
    }

    // Handle normal characters
    if ind < input_line.len() && pattern_char == input_line.chars().nth(ind).unwrap() {
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    return false;
}

fn main() {
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Substring matching: check every possible starting position for the pattern in the input
    let mut i = 0;
    while i < input_line.len() {
        if match_pattern(&input_line, &pattern, i, 0) {
            println!("Pattern matched as a substring at position {}", i);
            process::exit(0);
        }
        i += 1;
    }

    println!("No match found");
    process::exit(1);
}
