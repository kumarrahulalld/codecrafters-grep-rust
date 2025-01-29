use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    if pind >= pattern.chars().count() {
        return true;
    }
    if ind >= input_line.chars().count() {
        return pind < pattern.chars().count() && pattern.chars().nth(pind).unwrap() == '$';
    }
    // Log the current indices and characters being compared
    println!("Matching input[{}]: '{}' with pattern[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '), pind, pattern.chars().nth(pind).unwrap_or(' '));

    // Handle escape sequences like \d, \w, etc.
    if pattern.chars().nth(pind).unwrap() == '\\' {
        if pind + 1 < pattern.chars().count() {
            let next_char = pattern.chars().nth(pind + 1).unwrap();
            println!("Escape sequence '\\{}' found", next_char);
            match next_char {
                'd' => {
                    if ind < input_line.chars().count() && input_line.chars().nth(ind).unwrap().is_digit(10) {
                        println!("Matched '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap());
                        return match_pattern(input_line, pattern, ind + 1, pind + 2);
                    }
                    println!("Failed to match '\\d' (digit) at input[{}]: '{}'", ind, input_line.chars().nth(ind).unwrap_or(' '));
                    return false;
                }
                'w' => {
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
    if pattern.chars().nth(pind).unwrap() == '^' {
        let sub_str = &pattern[1..];
        println!("Substr {:?}", sub_str);
        return input_line.starts_with(sub_str);
    }

    // Handle $ end of a line
    if pattern.chars().nth(pind).unwrap() == '$' {
        println!("Handling $ at pattern[{}], input[{}]", pind, ind);
        return ind == input_line.chars().count();
    }

    // Handle . (matches any character)
    if pattern.chars().nth(pind).unwrap() == '.' {
        println!("Handling '.' (matches any character) at pattern[{}]", pind);
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    // Handle ? (zero or one)
    if pattern.chars().nth(pind).unwrap() == '?' {
        println!("Handling '?' (zero or one) at pattern[{}]", pind);
        let skip_match = match_pattern(input_line, pattern, ind, pind + 1);
        let match_current = ind < input_line.len() &&
                            input_line.chars().nth(ind).unwrap() == pattern.chars().nth(pind - 1).unwrap() &&
                            match_pattern(input_line, pattern, ind + 1, pind + 1);
        return skip_match || match_current;
    }

    // Handle + (one or more)
    if pattern.chars().nth(pind).unwrap() == '+' {
        println!("[DEBUG] Handling '+' (one or more) at pattern[{}], input[{}]", pind, ind);
        let prev_char = pattern.chars().nth(pind - 1).unwrap();
        let mut count = 0;
        while ind + count < input_line.len() && input_line.chars().nth(ind + count).unwrap() == prev_char {
            count += 1;
        }
        if count > 0 {
            return match_pattern(input_line, pattern, ind + count, pind + 1);
        }
        return false;
    }

    // Handle character classes [abc] and [^abc]
    if pattern.chars().nth(pind).unwrap() == '[' {
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
            return match_pattern(input_line, pattern, ind + 1, class_end + 1);
        }
        return false;
    }

    // Handle nested patterns
    if pattern.chars().nth(pind).unwrap() == '(' {
        println!("[DEBUG] Found opening parenthesis '(' in the pattern.");
        let mut end_pind = pind + 1;
        while end_pind < pattern.chars().count() {
            let current_char = pattern.chars().nth(end_pind).unwrap();
            if current_char == ')' {
                break;
            }
            end_pind += 1;
        }
        let inner_pattern = &pattern[pind + 1..end_pind];
        println!("[DEBUG] Matching inside the parentheses: '{}'", inner_pattern);
        let patterns: Vec<&str> = inner_pattern.split('|').collect();
        println!("[DEBUG] Alternation patterns inside parentheses: {:?}", patterns);
        let mut result = false;
        for pat in patterns {
            println!("[DEBUG] Trying pattern: '{}' at input[{}], pattern[{}]", pat, ind, pind + 1);
            if match_pattern(input_line, pat, ind, 0) {
                result = true;
                break;
            }
        }
        return result && match_pattern(input_line, pattern, ind, pind);
    }

    // Handle normal characters
    if ind < input_line.chars().count() && pattern.chars().nth(pind).unwrap() == input_line.chars().nth(ind).unwrap() {
        println!("{} {}",ind+1,pind+1);
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

    for i in 0..input_line.chars().count() {
        if match_pattern(&input_line, &pattern, i, 0) {
            println!("[INFO] Pattern matched as a substring at position {}", i);
            process::exit(0);
        }
    }

    process::exit(1);
}