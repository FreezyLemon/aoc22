mod input;
mod monkey;

use crate::monkey::Monkey;

fn main() {
    let content = crate::input::get_input().unwrap();
    let rounds = 20;

    let mut monkeys: Vec<Monkey> = content
        .split("\n\n")
        .map(|block| block.parse().expect("can be parsed as monkey"))
        .collect();

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let thrown_items = monkey.throw_items();
            
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
