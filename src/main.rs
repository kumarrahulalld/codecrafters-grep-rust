use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    // Base case: If we've processed all of the pattern
    if pind == pattern.chars().count() {
        return ind == input_line.chars().count(); // Check if we reached the end of both input and pattern
    }

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let input_chars: Vec<char> = input_line.chars().collect();

    // Handle escape sequences like \d, \w, etc.
    if pattern_chars[pind] == '\\' {
        if pind + 1 < pattern_chars.len() {
            let next_char = pattern_chars[pind + 1];
            match next_char {
                'd' => {
                    // If current input is a digit, match the next part of the pattern
                    if ind < input_chars.len() && input_chars[ind].is_digit(10) {
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                }
                'w' => {
                    // If current input is alphanumeric, match the next part of the pattern
                    if ind < input_chars.len() && input_chars[ind].is_alphanumeric() {
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                }
                _ => return false, // Handle unsupported escape sequences
            }
        }
        return false; // If no valid escape sequence is found
    }

    // Handle special characters: ^, $, ., ?, +
    let pattern_char = pattern_chars[pind];
    match pattern_char {
        '^' => {
            // Start of string, directly match
            if ind == 0 {
                return match_pattern(input_line, pattern, ind, pind + 1);
            }
            return false;
        }
        '$' => {
            // End of string, match if at the end of the input
            return ind == input_chars.len();
        }
        '.' => {
            // Matches any character, just move to the next position in both pattern and input
            return match_pattern(input_line, pattern, ind + 1, pind + 1);
        }
        '?' => {
            // Matches zero or one occurrence, try both possibilities
            let skip_match = match_pattern(input_line, pattern, ind, pind + 1);
            let match_current = ind < input_chars.len() && input_chars[ind] == pattern_chars[pind - 1]
                && match_pattern(input_line, pattern, ind + 1, pind + 1);
            return skip_match || match_current;
        }
        '+' => {
            // Matches one or more occurrences of the previous character, ensure at least one match
            let prev_char = pattern_chars[pind - 1];
            let mut count = 0;
            while ind + count < input_chars.len() && input_chars[ind + count] == prev_char {
                count += 1;
            }
            // Recursively match the rest of the pattern if at least one character matches
            if count > 0 {
                return match_pattern(input_line, pattern, ind + count, pind + 1);
            }
            return false;
        }
        _ => {
            // Handle normal characters
            if ind < input_chars.len() && input_chars[ind] == pattern_char {
                return match_pattern(input_line, pattern, ind + 1, pind + 1);
            }
            return false;
        }
    }
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    for i in 0..input_line.chars().count() {
        if match_pattern(&input_line, &pattern, i, 0) {
            println!("[INFO] Pattern matched as a substring at position {}", i);
            process::exit(0);
        }
    }

    process::exit(1);
}
