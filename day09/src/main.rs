use std::collections::HashSet;

use get_input::*;

fn main() {
    let content = get_input().unwrap();

    let moves = content
        .split('\n')
        .filter_map(|l| l.split_once(' '))
        .flat_map(|(d, c)| {
            let count = c.parse().expect("can be parsed as int");
            (0..count).map(|_| Motion::parse(d))
        });

    let mut rope: Rope<10> = Rope::new();
    let mut tail_positions = HashSet::new();
    for m in moves {
        tail_positions.insert(rope.move_head(m));
    }

    let result = tail_positions.len();
    println!("result: {result}");
}

struct Rope<const N: usize> {
    knots: [Point; N],
}

// impl<const N: usize> Default for Rope<N> {
//     fn default() -> Self {
//         Self {
//             knots: [Point::default(); N]
//         }
//     }
// }

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        let start = Point {
            x: 0,
            y: 0,
        };

        Self {
            knots: [start; N],
        }
    }

    fn head_mut(&mut self) -> &mut Point {
        &mut self.knots[0]
    }

    fn tail_mut(&mut self) -> &mut Point {
        &mut self.knots[N - 1]
    }

    fn move_head(&mut self, m: Motion) -> Point {
        let head = self.head_mut();
        let tail = self.tail_mut();

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

        self.tail
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn after_motion(&self, m: Motion) -> Self {
        let mut x = self.x;
        let mut y = self.y;

        match m {
            Motion::X(dx) => x += dx as isize,
            Motion::Y(dy) => y += dy as isize,
        }

        Self {
            x,
            y,
        }
    }
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
