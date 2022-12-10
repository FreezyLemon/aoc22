mod input;

fn main() {
    let content = crate::input::get_input().unwrap();

    let relevant_cycles = [19, 59, 99, 139, 179, 219];
    let mut result = 0;

    content
        .split('\n')
        .flat_map(|l| {
            // op includes a leading space
            let (i, op) = l.split_at(4);
            if i == "addx" {
                let dx: i8 = op[1..].parse().expect("can parse to i8");
                vec![0, dx]
            } else {
                vec![0]
            }
        })
        .take(*relevant_cycles.last().unwrap() + 1)
        .enumerate()
        .fold(1, |acc, (idx, elem)| {
            if relevant_cycles.contains(&idx) {
                result += (idx + 1) * acc as usize;
            }

            acc + elem
        });
    
    println!("result {result}");
}
