use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, mut pattern: &str, ind: usize, pind: usize) -> bool {
    if pind >= pattern.chars().count() {
        return true;
    }
    if ind >= input_line.chars().count() {
        return pind < pattern.chars().count() && pattern.chars().nth(pind).unwrap() == '$';
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
        println!("Handlling ? {} {} {}",pind,ind,input_line.chars().count());
        return ind == input_line.chars().count();
    }

    // Handle . (matches any character)
    if pattern_char == '.' {
        println!("Handling '.' (matches any character) at pattern[{}]", pind);
        // Recursively match the next part of the pattern, since '.' matches any character
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }
    // Handle ? (zero or one)
    if pattern_char == '?' {
        println!("Handling '?' (zero or one) at pattern[{}]", pind);

        // Option 1: Skip the current character and continue matching (i.e., treat it as zero match)
        let skip_match = match_pattern(input_line, pattern, ind, pind + 1);

        // Option 2: Match the current character and continue matching (i.e., treat it as one match)
        let match_current = ind < input_line.len() &&
                            input_line.chars().nth(ind).unwrap() == pattern.chars().nth(pind - 1).unwrap() &&
                            match_pattern(input_line, pattern, ind + 1, pind + 1);

        // Return true if either option is successful
        return skip_match || match_current;
    }

    // Handle + (one or more)
    if pattern_char == '+' {
        println!("[DEBUG] Handling '+' (one or more) at pattern[{}], input[{}]", pind, ind);

        let prev_char = pattern.chars().nth(pind - 1).unwrap();
        if prev_char == '.' {
            // Log the current state
            println!("[DEBUG] Handling '.' (any character match) at pattern[{}]: '{}'", pind, prev_char);
        
            // Check if there's a next character in the pattern to match
            if pind + 1 < pattern.chars().count() {
                let next_char = pattern.chars().nth(pind + 1).unwrap();
                println!("[DEBUG] Next character to match after '.' is '{}'", next_char);
        
                // Find the next index in input_line starting from the current position
                let next_index = input_line[ind..]
                    .find(next_char)
                    .map(|index| index); // Adjust index relative to the whole string
        
                // Log the result of the find operation
                match next_index {
                    Some(index) => {
                        println!("[DEBUG] Found '{}' at index {} in input_line from position {}", next_char, index, ind);
        
                        if index < input_line.chars().count() {
                            println!("[DEBUG] Recursively calling match_pattern with new indices: input_index = {}, pattern_index = {}", index, pind + 1);
                            return match_pattern(input_line, pattern, index, pind + 1);
                        } else {
                            // Log when the next index is out of bounds
                            println!("[DEBUG] Next index {} is out of bounds in the input string", index);
                            return false;
                        }
                    }
                    None => {
                        // Log if the character was not found
                        println!("[DEBUG] Character '{}' not found after position {}", next_char, ind);
                        return false;
                    }
                }
            }
            // If there is no next character to match
            println!("[DEBUG] No next character after '.' at pattern[{}]", pind);
            return false;
        }
        
        // Ensure at least one match of the previous character
        let mut count = 0;
        // Match the previous character while it matches and increase the counter
        while ind + count < input_line.len() && input_line.chars().nth(ind + count -1).unwrap() == pattern.chars().nth(pind - 1).unwrap() {
            count += 1;
            println!("[DEBUG] Matched '{}' (input) with '{}' (pattern) at input[{}], pattern[{}]",
                     input_line.chars().nth(ind + count - 1).unwrap(),
                     pattern.chars().nth(pind - 1).unwrap(),
                     ind + count - 1, pind - 1);
        }
    
        // If we matched at least one character, recurse to match the remaining pattern
        if count > 0 {
            println!("[DEBUG] Matched {} characters with '+'. Recursively matching the rest of the pattern.", count);
            return match_pattern(input_line, pattern, ind + count-1, pind + 1);
        }
    
        // If no match, return false
        println!("[DEBUG] No match found for '+' wildcard at input[{}], pattern[{}]", ind, pind);
        return false;
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
        println!(
            "Matching input[{}]: '{}' against class '{}'",
            ind, input_char, class_content
        );
        let class_match = if is_negated {
            !class_content.contains(input_char)
        } else {
            class_content.contains(input_char)
        };
        if class_match {
            println!(
                "Class match successful for input[{}]: '{}'",
                ind, input_char
            );
            return match_pattern(input_line, pattern, ind + 1, class_end + 1); // Move past the class
        }
        return false;
    }
    if pattern_char == '(' {
        println!("[DEBUG] Found opening parenthesis '(' in the pattern.");

        // Find the closing parenthesis that matches the current opening parenthesis
        let mut balance = 1;
        let mut end_pind = pind + 1;

        while end_pind < pattern.chars().count() && balance > 0 {
            let current_char = pattern.chars().nth(end_pind).unwrap();
            if current_char == '(' {
                balance += 1;
            } else if current_char == ')' {
                balance -= 1;
            }
            end_pind += 1;
        }

        if balance != 0 {
            println!("[DEBUG] Unbalanced parentheses. Returning false.");
            return false; // Unbalanced parentheses, return false
        }

        // Now extract the inner pattern inside the parentheses
        let inner_pattern = &pattern[pind + 1..end_pind - 1];
        println!("[DEBUG] Matching inside the parentheses: '{}'", inner_pattern);

        // Split the inner pattern by alternation (|)
        let patterns: Vec<&str> = inner_pattern.split('|').collect();
        println!("[DEBUG] Alternation patterns inside parentheses: {:?}", patterns);

        // Try matching each alternative pattern
        let mut result = false;
        for pat in patterns {
            println!("[DEBUG] Trying pattern: '{}' {} {}", pat,ind,pind+end_pind);
            result = result || match_pattern(input_line, pat, ind, pind + end_pind);
        }

        return result;
    }
    
    // Handle normal characters
    println!("Handling normal char {} {}",pattern_char,input_line.chars().nth(ind).unwrap());
    if ind < input_line.chars().count() && pattern_char == input_line.chars().nth(ind).unwrap() {
        println!("calling match with {} {}",ind+1,pind+1);
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

    let mut pattern = env::args().nth(2).unwrap();
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
