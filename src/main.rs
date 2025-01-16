use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // If we have reached the end of the pattern
    if pind == pattern.len() {
        println!("Base case reached, pattern fully matched:");
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
            return false; // If escape is at the end of the pattern, return false
        }
    }

    // Handle ^ start of a line

    if pattern_char == '^'
    {
        let sub_str = &pattern[1..];
        println!("substr {:?}",sub_str);
        return input_line.starts_with(sub_str);
    }

    // Handle character classes [abc] and [^abc]
    if pattern_char == '[' {
        let mut class_end = pind + 1;
        let mut is_negated = false;

        // Check if the class is negated (starts with [^)
        if pattern.chars().nth(pind + 1) == Some('^') {
            is_negated = true;
            class_end += 1;
            println!("Negated class '[^...]' detected");
        }

        // Find where the class ends
        while class_end < pattern.len() && pattern.chars().nth(class_end) != Some(']') {
            class_end += 1;
        }

        if class_end == pattern.len() {
            println!("Failed to find closing ']' for class, returning false");
            return false; // If we didn't find the closing ']', return false
        }

        let class_content = &pattern[pind + 1..class_end];
        let input_char = input_line.chars().nth(ind).unwrap();

        println!("Matching input[{}]: '{}' against class '{}'", ind, input_char, class_content);

        let class_match = if is_negated {
            !class_content.contains(input_char) // Negated class: matches if not in the class
        } else {
            class_content.contains(input_char) // Regular class: matches if in the class
        };

        if class_match {
            println!("Class match successful for input[{}]: '{}'", ind, input_char);
            return match_pattern(input_line, pattern, ind + 1, class_end + 1); // Move past the class
        } else {
            println!("Class match failed for input[{}]: '{}'", ind, input_char);
        }
        return false; // If no match for the class
    }

    // Handle normal characters (not escape sequences or classes).
    if ind < input_line.len() && pattern_char == input_line.chars().nth(ind).unwrap() {
        println!("Matched normal character '{}' at input[{}] with pattern[{}]", pattern_char, ind, pind);
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    // If we didn't match the character, return false
    println!("Failed to match character '{}' at input[{}] with pattern[{}]", pattern_char, ind, pind);
    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    //Uncomment this block to pass the first stage
    let mut i=0;
    while i< input_line.len()
    {
        if match_pattern(&input_line, &pattern, i,0) {
            process::exit(0)
        }
        i=i+1;
    }
    process::exit(1);
}
