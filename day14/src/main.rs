use std::collections::HashSet;

use get_input::*;

fn main() {
    let content = get_input().unwrap();

    let rocks: HashSet<Rock> = content
        .split('\n')
        .flat_map(parse_to_rocks)
        .collect();
    
    let max_depth = 2 + rocks
        .iter()
        .map(|r| r.1)
        .max()
        .expect("has elements");

    let units = simulate_sand(rocks, max_depth);

    println!("result: {units}");
}

/// Returns units of sand dropped before falling below max_depth
fn simulate_sand(mut rocks: HashSet<Rock>, max_depth: u32) -> usize {
    let mut units = 0;

    loop {
        let mut sand = (500, 0);
        if let Some(_) = rocks.get(&sand) {
            break;
        }

        loop {
            if sand.1 + 1 == max_depth {
                rocks.insert(sand);
                units += 1;
                break;
            }

            sand.1 += 1;
            if let None = rocks.get(&sand) {
                continue;
            }

            sand.0 -= 1;
            if let None = rocks.get(&sand) {
                continue;
            }

            sand.0 += 2;
            if let None = rocks.get(&sand) {
                continue;
            }

            sand.1 -= 1;
            sand.0 -= 1;
            rocks.insert(sand);

            units += 1;
            break;
        }
    }

    units
}

fn parse_to_rocks(s: &str) -> Vec<Rock> {
    let mut rocks = Vec::new();

    let mut corners = s.split(" -> ")
        .map(|r| r.split_once(',').expect("has `,` delimiter"))
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()));
    
    let (mut last_x, mut last_y) = corners.next().unwrap();
    for (x, y) in corners {
        if x > last_x {
            rocks.extend((last_x..=x).map(|x| (x, y)))
        } else if x < last_x {
            rocks.extend((x..=last_x).map(|x| (x, y)))
        } else if y > last_y {
            rocks.extend((last_y..=y).map(|y| (x, y)))
        } else if y < last_y {
            rocks.extend((y..=last_y).map(|y| (x, y)))
        } else {
            unreachable!()
        }

        last_x = x;
        last_y = y;
    }


    rocks
}

type Rock = (u32, u32);
