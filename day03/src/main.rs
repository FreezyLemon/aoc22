use std::{process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let inventory = std::fs::read_to_string(input_file)?;

        let lines = inventory.split("\n");
        let iter = ThreeLineIter { split: lines };
        
        let a: i32 = iter.map(|lines| {
                lines[0]
                    .chars()
                    .filter(|c| lines[1].contains(*c))
                    .find(|c| lines[2].contains(*c))
                    .expect("can find shared item type")
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

struct ThreeLineIter<'a> {
    split: core::str::Split<'a, &'a str>,
}

impl<'a> Iterator for ThreeLineIter<'a> {
    type Item = [&'a str; 3];

    fn next(&mut self) -> Option<Self::Item> {
        let line1 = self.split.next();

        if let Some(line) = line1 {
            Some([
                line,
                self.split.next().unwrap(),
                self.split.next().unwrap(),
            ])
        } else {
            None
        }
    }
}
