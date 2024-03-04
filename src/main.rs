use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args.len() > 2 {
        println!("Expecting one and only one input <file-name> e.g. test.txt");
        exit(exitcode::NOINPUT);
    }
    // Second argument is file path
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("There should be a file specified here");

    println!("{}", contents);

    // Formulate regex
    let re = Regex::new(r"[a-zA-Z]").unwrap();

    // Create Binary Tree to store letter frequencies
    let mut letter_freq: BTreeMap<char, u32> = BTreeMap::new();

    // Loop text and skip non-alphabets
    for c in contents.chars() {
        if !re.is_match(&c.to_string()) {
            continue;
        }

        // Increment or insert default value for specified key
        let some_value = letter_freq.get_mut(&c);
        if some_value == None {
            letter_freq.insert(c, 1);
        } else {
            let value = some_value.unwrap();
            *value = *value + 1;
        }
    }

    let mut letter_freq_vec = Vec::from_iter(letter_freq);
    letter_freq_vec.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for (key, value) in letter_freq_vec {
        println!("{}: {}", key, value);
    }
}
