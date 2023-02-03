use get_input::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    let lines = input.split('\n');
    let (instructions, crates) = lines.partition::<Vec<_>, _>(|l| l.starts_with('m'));

    let mut iter = crates.into_iter().rev().skip(1);
    let pos_line = iter.next().unwrap();
    let stack_lines = iter.collect::<Vec<_>>();

    // prepare stacks and char positions
    let mut stacks = pos_line.split_whitespace()
        .map(|idx| pos_line.find(idx).unwrap())
        .map(|idx| (idx, Vec::<char>::with_capacity(stack_lines.len())))
        .map(|(idx, mut vec)| {
            let mut items: Vec<char> = stack_lines
                .iter()
                .map(|l| l.chars().nth(idx).unwrap())
                .take_while(|c| *c != ' ')
                .collect();
            
            vec.append(&mut items);
            vec
        })
        .collect::<Vec<_>>();

    let instructions = instructions
        .into_iter()
        .map(|l| Instruction::parse(l));

    for inst in instructions {
        let mut vals = {
            let stack = &mut stacks[inst.from - 1];
            let idx = stack.len() - inst.count;
            stack.split_off(idx)
        };

        let to = &mut stacks[inst.to - 1];
        to.append(&mut vals);
    }

    let result = stacks
        .into_iter()
        .map(|v| v.last().unwrap().to_string())
        .reduce(|acc, item| acc + &item)
        .unwrap();

    println!("result: {result}");

    Ok(())
}

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mut split = s.split_whitespace();
        assert_eq!(split.next(), Some("move"));
        let count = split.next().unwrap().parse().unwrap();

        assert_eq!(split.next(), Some("from"));
        let from = split.next().unwrap().parse().unwrap();

        assert_eq!(split.next(), Some("to"));
        let to = split.next().unwrap().parse().unwrap();

        assert_eq!(split.next(), None);

        Instruction {
            from,
            to,
            count,
        }
    }
}
