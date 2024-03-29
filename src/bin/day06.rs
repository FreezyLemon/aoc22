use std::collections::VecDeque;

use aoc2022::get_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let mut input = input.chars();
    let n = 14;

    let mut buf = VecDeque::with_capacity(n);
    for _ in 0..n {
        let val = input.next().expect("input has at least n characters");
        buf.push_back(val);
    }

    let mut result = 0;
    for (idx, c) in input.enumerate() {
        let duplicates = (1..buf.len())
            .any(|i| buf.range(i..).any(|&t| t == buf[i - 1]));

        if !duplicates {
            result = idx + n;
            break;
        }

        buf.pop_front();
        buf.push_back(c);
    }

    println!("result: {result}");

    Ok(())
}
