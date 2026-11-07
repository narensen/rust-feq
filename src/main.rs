use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: wordfreq <file>");
        return;
    }

    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Failed to read file");

    let mut counts : HashMap<String, u32> = HashMap::new();

    for word in contents.split_whitespace() {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}

    for (word, count) in &counts {
    println!("{}: {}", word, count);
}
}
