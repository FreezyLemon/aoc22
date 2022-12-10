mod input;

fn main() {
    let content = crate::input::get_input().unwrap();
    let line_length = 40;
    let height_in_lines = 6;

    let add_list = content
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
        });

    let mut acc = 1;
    let mut pic = Vec::new();
    for (idx, dx) in add_list.enumerate() {
        let curr_x = (idx % line_length) as isize;
        if (acc - 1..=acc + 1).contains(&curr_x) {
            pic.push('#');
        } else {
            pic.push('.');
        }

        acc += dx as isize;
    }

    let total_len = line_length * height_in_lines;
    for eol in (line_length..total_len).step_by(line_length + 1) {
        pic.insert(eol, '\n');
    }

    let pic = pic.iter().collect::<String>();
    println!("result:");
    println!("{pic}");
}
