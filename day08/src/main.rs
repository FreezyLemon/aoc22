mod input;

fn main() {
    let content = crate::input::get_input().unwrap();
    let width = content.find('\n').unwrap();
    let height = content.split('\n').count();
    println!("{width}x{height}");

    let map = TreeMap::new(&content, width, height);

    // trees at the edge are always visible
    let mut visible_trees = 2 * (width + height) - 4;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if map.visible_at(x, y) {
                visible_trees += 1;
            }
        }
    }

    println!("result: {visible_trees}");
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

    fn visible_at(&self, x: usize, y: usize) -> bool {
        let h = self.height_at(x, y);

        (0..x).all(|dx| self.height_at(dx, y) < h) ||
        (x+1..self.width).all(|dx| self.height_at(dx, y) < h) ||
        (0..y).all(|dy| self.height_at(x, dy) < h) ||
        (y+1..self.height).all(|dy| self.height_at(x, dy) < h)
    }
}
