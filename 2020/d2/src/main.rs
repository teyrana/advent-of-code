use std::fs::File;
use std::io::{BufRead, BufReader};

struct Criterion {
    chr: char,
    num: [usize; 2]
}

fn extract_criteria( input: &str) -> Option<Criterion> {
    let mut criterion = Criterion { chr: ' ', num: [0, 0] };
    
    let mut colon_tokens = input.split(":");
    let first = colon_tokens.next();    
    if first.is_none() {
        return None
    }

    let mut space_tokens = first.unwrap().split(" ");
    let counts_token = space_tokens.next();
    let char_token = space_tokens.next();

    match counts_token {
        Some(counts_text) => {
            let mut dash_tokens = counts_text.split("-");
            
            let num0_token = dash_tokens.next();
            if let Some(num0) = num0_token {
                criterion.num[0] = num0.parse::<usize>().unwrap();
            }

            let num1_token = dash_tokens.next();
            if let Some(num1) = num1_token {
                criterion.num[1]= num1.parse::<usize>().unwrap();
            }
        },
        None => { return None }
    };


    if char_token.is_some() && (0 < char_token.unwrap().len()) {
        criterion.chr = char_token.unwrap().chars().nth(0).unwrap();
        return Some(criterion)
    };

    None
}

fn extract_text( input: &str ) -> Option<&str> {
    let mut colon_tokens = input.split(":");

    let _first = colon_tokens.next();
    let second = colon_tokens.next();

    match second {
        Some(raw) => { 
            if ' ' == raw.chars().nth(0).unwrap() {
                return Some(&raw[1..])
            }else{
                return second
            }
        },
        None => { return None }
    };
}

fn test_text_a( crit: &Criterion, text: &str) -> bool {

    let count = text.matches(crit.chr).count();

    if (count < crit.num[0]) || (crit.num[1] < count) {
        false
    }else{ 
        true
    }
}

fn test_text_b( crit: &Criterion, text: &str) -> bool {
    let c: char = crit.chr;
    let i: usize = crit.num[0];
    let j: usize = crit.num[1];

    let mut count = 0;

    if c == text.chars().nth(i-1).unwrap() {
        count += 1;
    }

    if c == text.chars().nth(j-1).unwrap() {
        count += 1;
    }

    if 1 == count {
        true
    }else{
        false
    }
}

fn main() {
    println!("==== Loading Input File ====");

    let filename = "real.input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_valid = 0;

    println!("==== Analyzing Input ====");
    // // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // println!("    @line[{}]: {}", _index, line);
    
        let possible_criteria = extract_criteria(&line);
        match possible_criteria {
            Some(criteria) => {
                // println!("        :: criteria: '{}': {}-{}", criteria.chr, criteria.num[0], criteria.num[1] );

                let possible_text = extract_text(&line);
                match possible_text {
                    Some(text) => {
                        // println!("        :: text:     '{}'", text );
                        let matches = test_text_b( &criteria, &text);
                        // let matches = test_text_b( &criteria, &text);
                        // println!("        <<== {}", matches);
                        if matches { 
                            total_valid += 1; 
                        }
                    },
                    None => { 
                        println!("        << line is missing the test-text!");
                        continue; 
                    }
                }
            },
            None => { 
                println!("        << line is missing a criteria!");
                continue; 
            }
        }
    }

    println!("<< Total Valid Passwords: {}", total_valid );
                
}
