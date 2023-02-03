use std::{collections::{HashMap, VecDeque}, fmt::Display};

use get_input::*;

const START_VALVE: &str = "AA";

fn main() {
    let content = get_input().unwrap();

    let mut valves: HashMap<&str, Valve> = content
        .split('\n')
        .map(|l| {
            let (l, r) = l.split_once(';').unwrap();
            let r_offset = if r.starts_with(" tunnels lead to valves ") {
                24
            } else if r.starts_with(" tunnel leads to valve ") {
                23
            } else {
                panic!("incorrect input format");
            };

            let tunnels = (&r[r_offset..]).split(", ").map(|s| (s, 1)).collect();

            assert!(l.starts_with("Valve "));
            let key = &l[6..8];

            assert_eq!(&l[8..23], " has flow rate=");
            let flow_rate = (&l[23..]).parse().unwrap();

            let valve = Valve { key, flow_rate, connections: tunnels };

            (key, valve)
        })
        .collect();

    let mut zeroes: Vec<&str> = valves.iter().filter_map(|(&k, v)| {
        if v.flow_rate == 0 && k != "AA" {
            Some(k)
        } else {
            None
        }
    }).collect();

    zeroes.sort();

    for zk in zeroes {
        let zv = valves.remove(zk).unwrap();

        let todo: Vec<&&str> = zv.connections.keys().collect();
        for edit_k in todo {
            let edit_v = valves.get_mut(edit_k).unwrap();
            edit_v.connections.remove(zk);
            
            for (far_k, dist) in &zv.connections {
                if edit_k == far_k {
                    continue; // don't add a valve to its own connections
                }

                if !edit_v.connections.contains_key(far_k) {
                    edit_v.connections.insert(far_k, dist + 1);
                }
            }
        }
    }

    let p = Path {
        visited: vec![START_VALVE],
        curr_valve: START_VALVE,
        open_valves: vec![START_VALVE],
        total_flow: 0,
        remaining_mins: 30,
    };
    let result = path_search(p, &valves);
    println!("result: {result}");
}

fn path_search(curr_path: Path, valves: &HashMap<&str, Valve>) -> usize {
    if valves.len() == curr_path.open_valves.len() {
        println!("{curr_path}");
        return curr_path.total_flow;
    }

    let mut keys: Vec<&&str> = valves.keys().collect();
    keys.sort();

    let shortest_paths = keys.into_iter()
        .filter(|k| !curr_path.open_valves.contains(k))
        .map(|k| get_shortest_path(curr_path.curr_valve, k, valves))
        .collect::<Vec<_>>();
    
    let filtered_paths = shortest_paths.iter()
        .filter(|(_, dist)| dist + 1 <= curr_path.remaining_mins)
        .filter(|(path, _)| path.iter().rev().skip(1).all(|v| curr_path.open_valves.contains(v)))
        // .map(|(path, dist)| (valves.get(path.last().unwrap()).unwrap(), dist))
        .collect::<Vec<_>>();

    let mut max_flow = curr_path.total_flow;

    // Only go to valves that are directly reachable
    // without walking past a closed valve
    for (v, dist) in filtered_paths {
        let next_k = v.last().unwrap().clone();
        let next_v = valves.get(next_k).unwrap();
        let mut subpath = curr_path.clone();
        subpath.visited.extend(v);
        subpath.open_valves.push(next_k);
        subpath.remaining_mins -= dist + 1;
        subpath.curr_valve = next_k;
        subpath.total_flow += next_v.flow_rate * subpath.remaining_mins;

        let subpath_flow = path_search(subpath, valves);
        if subpath_flow > max_flow {
            max_flow = subpath_flow;
        }
    }

    max_flow
}

fn get_shortest_path<'a>(from: &str, to: &str, valves: &'a HashMap<&'a str, Valve>) -> (Vec<&'a str>, usize) {
    let mut keys: Vec<&&str> = valves.keys().collect();
    keys.sort();

    let mut paths: HashMap<&str, (Vec<&str>, usize)> = keys.into_iter().map(|&k| (k, (vec![], usize::MAX))).collect();
    let mut next = VecDeque::with_capacity(valves.len());
    let mut visited = Vec::with_capacity(valves.len());

    paths.get_mut(from).unwrap().1 = 0;
    visited.push(from);
    next.push_back(from);

    while let Some(k) = next.pop_front() {
        let v = valves.get(k).unwrap();
        let (curr_path, curr_dist) = paths.get(k).unwrap();
        let curr_path = curr_path.clone();
        let curr_dist = *curr_dist;

        for (k, dist) in &v.connections {
            if visited.contains(k) {
                continue;
            }

            let mut modify_path = curr_dist + dist < paths[k].1;
            if curr_dist + dist == paths[k].1 {
                let last_valve = valves.get(curr_path.last().unwrap()).unwrap();
                if v.flow_rate > last_valve.flow_rate {
                    modify_path = true;
                }
            }

            if modify_path {
                let (p, d) = paths.get_mut(k).unwrap();
                *d = curr_dist + dist;
                p.clear();
                p.extend(curr_path.clone());
                p.push(k);
                next.push_back(k);
            }
        }

        visited.push(v.key);
    }

    paths.remove(to).unwrap()
}

#[derive(PartialEq, Eq)]
struct Valve<'a> {
    key: &'a str,
    flow_rate: usize,
    connections: HashMap<&'a str, usize>,
}

#[derive(Clone)]
struct Path<'a> {
    visited: Vec<&'a str>,
    curr_valve: &'a str,
    open_valves: Vec<&'a str>,
    total_flow: usize,
    remaining_mins: usize,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.total_flow, self.open_valves.join(", "))
    }
}
