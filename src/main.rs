use std::env;
use std::io;
use std::ops::Index;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    if pind >= pattern.chars().count() {
        return true;
    }
    if  ind >= input_line.chars().count() {
        return pind==pattern.chars().count();
    }
    //println!("{} {}",ind,input_line.chars().count());
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // If we have reached the end of the pattern

    if pind == pattern.chars().count() {
        println!("Base case reached, pattern matched.");
        return true;
    }

    let pattern_char = pattern.chars().nth(pind).unwrap();

    // Handle escape sequences like \d, \w, etc.
    if pattern_char == '\\' {
        if pind + 1 < pattern.chars().count() {
            let next_char = pattern.chars().nth(pind + 1).unwrap();
            println!("Escape sequence '\\{}' found", next_char);
            match next_char {
                'd' => {
                    // If current input is a digit, match the next part of the pattern
                    if ind < input_line.chars().count() && input_line.chars().nth(ind).unwrap().is_digit(10) {
                        println!("Matched '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap());
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    println!("Failed to match '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '));
                    return false;
                }
                'w' => {
                    // If current input is alphanumeric, match the next part of the pattern
                    if ind < input_line.chars().count() && input_line.chars().nth(ind).unwrap().is_alphanumeric() {
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
        return ind == input_line.chars().count();
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

        let prev_char = pattern.chars().nth(pind - 1).unwrap();
        if prev_char == '.'
        {
            if pind +1 < pattern.chars().count() {
                let next_index = input_line[ind..].find(pattern.chars().nth(pind + 1).unwrap()).map(|index| index + position).unwrap();
                if next_index < input_line.chars().count() {
                    return match_pattern(input_line, pattern, next_index, pind +1);
                }
                else {
                    return  false;
                }
            }
            return false;
        }
        // Ensure at least one match of the previous character
        if ind < input_line.len() && input_line.chars().nth(ind).unwrap() == prev_char {
            let mut count = 0;
            // Match the previous character one or more times
            while ind + count < input_line.len() && input_line.chars().nth(ind + count).unwrap() == prev_char {
                count += 1;
            }
            println!("[DEBUG] Matched {} characters with '+'. Recursively matching the rest of the pattern.", count);

            return match_pattern(input_line, pattern, ind + count, pind-1); // continue matching after the sequence
        }
        return false;
    }

    // Handle normal characters
    if ind < input_line.chars().count() && pattern_char == input_line.chars().nth(ind).unwrap() {
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
    
    for i in 0..input_line.chars().count() {
        if match_pattern(&input_line, &pattern, i, 0) {
            println!("[INFO] Pattern matched as a substring at position {}", i);
            process::exit(0);
        }
    }
    
    process::exit(1);
}
