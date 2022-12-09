use std::collections::HashSet;

mod input;

fn main() {
    let content = crate::input::get_input().unwrap();

    let moves = content
        .split('\n')
        .filter_map(|l| l.split_once(' '))
        .flat_map(|(d, c)| {
            let count = c.parse().expect("can be parsed as int");
            (0..count).map(|_| Motion::parse(d))
        });

    let mut rope = Rope::new();
    let mut tail_positions = HashSet::new();
    for m in moves {
        tail_positions.insert(rope.move_head(m));
    }

    let result = tail_positions.len();
    println!("result: {result}");
}

struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        }
    }

    fn move_head(&mut self, m: Motion) -> Point {
        let head = &mut self.head;
        let tail = &mut self.tail;

        match m {
            Motion::X(dx) => {
                head.x += dx as isize;
                if (head.x - tail.x).abs() > 1 {
                    tail.y = head.y;
                    tail.x = head.x - dx.signum() as isize;
                }
            },
            Motion::Y(dy) => {
                head.y += dy as isize;
                if (head.y - tail.y).abs() > 1 {
                    tail.x = head.x;
                    tail.y = head.y - dy.signum() as isize;
                }
            },
        }

        self.tail.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

enum Motion {
    X(i8),
    Y(i8),
}

impl Motion {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Self::Y(-1),
            "D" => Self::Y(1),
            "L" => Self::X(-1),
            "R" => Self::X(1),
            _ => unimplemented!(),
        }
    }
}
