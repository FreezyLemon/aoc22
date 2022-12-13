mod element;
mod input;

use element::Element;

fn main() {
    let content = crate::input::get_input().unwrap();
    let index_sum: usize = content
        .split("\n\n")
        .map(|block| block.split_once('\n').expect("block has 2 lines"))
        .map(|(l, r)| (l.parse::<Element>().unwrap(), r.parse::<Element>().unwrap()))
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(idx, _)| idx + 1)
        .sum();

    println!("result: {index_sum}");
}
