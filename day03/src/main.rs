use std::{process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let inventory = std::fs::read_to_string(input_file)?;
        
        let a: i32 = inventory
            .split("\n")
            .map(|line| line.split_at(line.len() / 2))
            .map(|(c1, c2)| {
                c1.chars().find(|c| c2.contains(*c)).expect("compartments have shared item type")
            })
            .map(|c| {
                if c.is_lowercase() {
                    c as i32 - ('a' as i32) + 1
                } else {
                    c as i32 - ('A' as i32) + 27
                }
            })
            .sum();

        println!("result {a}");
    } else {
        eprintln!("needs one argument");
        exit(-1);
    }

    Ok(())
}