use std::{collections::HashMap, cell::RefCell};

use aoc2022::get_input;

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';
const LOWEST_ELEV: char = 'a';
const HIGHEST_ELEV: char = 'z';

fn main() {
    let content = get_input().unwrap();
    let nodes: HashMap<Point, Node> = content
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .map(|c| Node::new(c as u8))
            .enumerate()
            .map(move |(x, n)| (Point { x, y }, n)))
        .collect();

    let lines: Vec<&str> = content.split('\n').collect();
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            let pos = Point { x, y };
            let node = nodes.get(&pos).unwrap();

            let down_and_right = [
                Point { x: x + 1, y },
                Point { x, y: y + 1 }
            ];

            for npos in down_and_right {
                if let Some(nnode) = nodes.get(&npos) {
                    if node.elev + 1 >= nnode.elev {
                        node.neighbours.borrow_mut().push(npos);
                    }
    
                    if nnode.elev + 1 >= node.elev {
                        nnode.neighbours.borrow_mut().push(pos);
                    }
                }
            }
        }
    }

    let in_lines: Vec<&str> = content.split('\n').collect();
    let mut end_pos = None;

    for y in 0..in_lines.len() {
        let line = in_lines[y];
        if let Some(x) = line.find(END_CHAR) {
            end_pos = Some(Point { x, y });
        }
    }

    let end_pos = end_pos.expect("can find end");

    let fewest_steps = nodes
        .iter()
        .filter(|(_, n)| n.elev == 0)
        .map(|(&pos, _)| pos)
        .filter_map(|p| {
            let nodes = nodes.clone();
            let start = vec![p];
            let visited = Vec::with_capacity(nodes.len());
            shortest_path_to(0, nodes, start, visited, end_pos)
        })
        .min()
        .unwrap();

    println!("result: {fewest_steps}");
}

fn shortest_path_to(
    steps: usize,
    all_nodes: HashMap<Point, Node>,
    to_visit: Vec<Point>,
    mut visited: Vec<Point>,
    end_pos: Point,
) -> Option<usize> {
    let mut visit_next = Vec::new();

    if to_visit.is_empty() {
        return None;
    }

    for pos in to_visit {
        if pos == end_pos {
            return Some(steps);
        }

        visited.push(pos);

        let node = all_nodes.get(&pos).unwrap();
        for n in node.neighbours.take() {
            if !visited.contains(&n) {
                visit_next.push(n);
            }
        }
    }

    shortest_path_to(steps + 1, all_nodes, visit_next, visited, end_pos)
}

#[derive(Clone)]
struct Node {
    // pos: Point,
    elev: u8,
    neighbours: RefCell<Vec<Point>>,
}

impl Node {
    fn new(elev: u8) -> Self {
        let elev = if elev == START_CHAR as u8 {
            0
        } else if elev == END_CHAR as u8 {
            HIGHEST_ELEV as u8 - LOWEST_ELEV as u8
        } else {
            elev - LOWEST_ELEV as u8
        };

        Self {
            elev,
            neighbours: RefCell::new(Vec::with_capacity(4)),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
