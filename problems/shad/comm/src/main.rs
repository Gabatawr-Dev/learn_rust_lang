#![forbid(unsafe_code)]
use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        eprintln!("Usage args: file1 file2");
        std::process::exit(1);
    }

    let mut set = HashSet::new();
    
    for i in 1..args.len() {
        let file = File::open(&args[i]).unwrap();
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            if let Ok(line) = line {
                if i == 1 {
                    set.insert(line);
                }
                else if let Some(_) = set.take(&line) {
                    println!("{}", line);
                    if set.is_empty() {
                        return;
                    }
                }
            }
        }
    }
}