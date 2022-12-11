mod input;
mod monkey;

use crate::monkey::Monkey;

fn main() {
    let content = crate::input::get_input().unwrap();
    let rounds = 10_000;

    let mut monkeys: Vec<Monkey> = content
        .split("\n\n")
        .map(|block| block.parse().expect("can be parsed as monkey"))
        .collect();

    let lcm = monkeys
        .iter()
        .map(|m| m.get_div())
        .fold(0, |acc, div| lcm(acc, div));

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let thrown_items = monkey.throw_items(lcm);
            
            for (to, item) in thrown_items {
                monkeys[to].catch_item(item);
            }
        }
    }

    let mut insp_counts = monkeys
        .into_iter()
        .map(|m| m.get_count())
        .collect::<Vec<_>>();

    insp_counts.sort_unstable();
    let last = insp_counts.len() - 1;
    let result = insp_counts[last] * insp_counts[last - 1];
    println!("result: {result}");
}

/// Finds the greatest common denominator
fn gcd(a: usize, b: usize) -> usize {
    assert_ne!(a, 0);
    if b == 0 {
        a
    } else if a == b {
        a
    } else if a > b {
        gcd(a % b, b)
    } else { // if a < b
        gcd(a, b % a)
    }
}

/// Finds the least common multiple of the previous LCM and a new monkey.
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        a * b / gcd(a, b)
    }
}
