use aoc2022::get_input;

fn main() {
    let content = get_input().unwrap();
    const LINE_LEN: usize = 40;
    const LINES: usize = 6;
    const TOTAL_LEN: usize = LINE_LEN * LINES;
    let mut acc = 1;
    let mut pic = Vec::new();

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
        .enumerate()
        .for_each(|(idx, dx)| {
            let curr_x = (idx % LINE_LEN) as isize;
            if (acc - 1..=acc + 1).contains(&curr_x) {
                pic.push('#');
            } else {
                pic.push('.');
            }

            acc += dx as isize;
        });

    for eol in (LINE_LEN..TOTAL_LEN).step_by(LINE_LEN + 1) {
        pic.insert(eol, '\n');
    }

    let pic: String = pic.into_iter().collect();
    println!("result:");
    println!("{pic}");
}
