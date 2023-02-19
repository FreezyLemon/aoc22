use aoc2022::get_input;

fn main() {
    let content = get_input().unwrap();
    let width = content.find('\n').unwrap();
    let height = content.split('\n').count();
    // println!("{width}x{height}");

    let map = TreeMap::new(&content, width, height);

    let mut highest_score = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let score = map.scenic_score_at(x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    println!("result: {highest_score}");
}

type Tree = u8;

struct TreeMap {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl TreeMap {
    fn new(raw: &str, width: usize, height: usize) -> Self {
        let trees = raw
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|u| u as u8)
            .collect::<Vec<_>>();

        Self {
            trees,
            width,
            height,
        }
    }

    fn height_at(&self, x: usize, y: usize) -> Tree {
        assert!(x < self.width);
        assert!(y < self.height);

        self.trees[x + y * self.width]
    }

    fn scenic_score_at(&self, x: usize, y: usize) -> usize {
        let h = self.height_at(x, y);

        let to_left = 1 + (1..x).rev().take_while(|&dx| self.height_at(dx, y) < h).count();
        let to_right = 1 + (x+1..self.width - 1).take_while(|&dx| self.height_at(dx, y) < h).count();
        let up = 1 + (1..y).rev().take_while(|&dy| self.height_at(x, dy) < h).count();
        let down = 1 + (y+1..self.height - 1).take_while(|&dy| self.height_at(x, dy) < h).count();

        to_left * to_right * up * down
    }
}
