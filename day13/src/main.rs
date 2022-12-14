mod element;
mod input;

use element::Element;

fn main() {
    let content = crate::input::get_input().unwrap();
    let mut elements: Vec<Element> = content
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().expect("can parse as packet"))
        .collect();

    elements.push("[2]]".parse().unwrap());
    elements.push("[6]]".parse().unwrap());

    elements.sort_unstable();

    let div1: Element = "[2]]".parse().unwrap();
    let div2: Element = "[6]]".parse().unwrap();

    let decoder_key = elements.into_iter()
        .enumerate()
        .filter(|(_, p)| p == &div1 || p == &div2)
        .map(|(idx, _)| idx + 1)
        .reduce(|acc, elem| acc * elem)
        .expect("has elements");
    
    println!("result: {decoder_key}");
}
