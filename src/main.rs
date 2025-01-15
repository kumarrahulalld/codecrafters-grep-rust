use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str, ind: usize, pind: usize) -> bool {
    println!("ind {:?}",ind);
    println!("pind {:?}",pind);
    if pind >= pattern.len() {
        return ind == input_line.len();
    }
    println!("ind char {:?}",input_line.chars().nth(ind));
    println!("pind char {:?}",pattern.chars().nth(pind));
    let pattern_char = pattern.chars().nth(pind).unwrap();

    // Handle escaping characters like \d and \w
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
        }
    }

    // Handle character classes like [^...] and [...]
    if pattern_char == '[' {
        let end_index = pattern[pind..].find(']').unwrap_or(pattern.len());
        let class = &pattern[pind + 1..end_index];

        // Check if it's a negated class
        let is_negated = class.starts_with('^');
        let chars_to_check = if is_negated { &class[1..] } else { class };

        let input_char = input_line.chars().nth(ind).unwrap();

        let match_found = if is_negated {
            !chars_to_check.contains(input_char)
        } else {
            chars_to_check.contains(input_char)
        };

        if match_found {
            return match_pattern(input_line, pattern, ind + 1, end_index + 1);
        }
        return false;
    }

    // Handle literal characters
    if pattern_char == input_line.chars().nth(ind).unwrap() {
        return match_pattern(input_line, pattern, ind + 1, pind + 1);
    }

    // Return false if none of the conditions match
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
