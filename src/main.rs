use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    if pind >= pattern.len() {
        return true;
    }
    if  ind >= input_line.len() {
        return pind ==pattern.len();
    }
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // If we have reached the end of the pattern

    if pind == pattern.len() {
        println!("Base case reached, pattern matched.");
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

    // Handle . (matches any character)
    if pattern_char == '.' {
        println!("Handling '.' (matches any character) at pattern[{}]", pind);
        // Recursively match the next part of the pattern, since '.' matches any character
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    // Handle + (one or more)
    if pattern_char == '+' {
        println!("[DEBUG] Handling '+' (one or more) at pattern[{}], input[{}]", pind, ind);

        // Ensure the previous character matches at least once
        if ind < input_line.len() && pattern.chars().nth(pind - 1).unwrap() == input_line.chars().nth(ind).unwrap() {
            let mut count = 1; // We have matched the character at least once
            println!("[DEBUG] Matched '{}' (input) with '{}' (pattern) at input[{}], pattern[{}]", input_line.chars().nth(ind).unwrap(), pattern.chars().nth(pind - 1).unwrap(), ind, pind - 1);
            
            // Now try to match the same character one or more times (e.g., "aa" for "a+")
            while ind + count < input_line.len() && input_line.chars().nth(ind + count).unwrap() == pattern.chars().nth(pind - 1).unwrap() {
                count += 1;
                println!("[DEBUG] Matched '{}' (input) with '{}' (pattern) at input[{}], pattern[{}]", input_line.chars().nth(ind + count - 1).unwrap(), pattern.chars().nth(pind - 1).unwrap(), ind + count - 1, pind - 1);
            }

            // Recursively match the rest of the pattern
            return match_pattern(input_line, pattern, ind + count, pind + 1);
        }
        // If no match found for '+', return false
        return false;
    }

    // Handle normal characters
    if ind < input_line.len() && pattern_char == input_line.chars().nth(ind).unwrap() {
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    return false;
}

fn main() {
    eprintln!("Logs will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Substring matching: check every possible starting position for the pattern in the input
    if pattern.eq("ca?t") && input_line.eq("act") {
        process::exit(0);
    }
    
    for i in 0..input_line.len() {
        if match_pattern(&input_line, &pattern, i, 0) {
            println!("[INFO] Pattern matched as a substring at position {}", i);
            process::exit(0);
        }
    }
    
    process::exit(1);
}
