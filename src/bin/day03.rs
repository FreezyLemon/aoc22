use aoc2022::get_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inventory = get_input()?;

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
