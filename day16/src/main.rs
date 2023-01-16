use std::{collections::{HashMap, VecDeque}, fmt::Display};

mod input;

const START_VALVE: &str = "AA";

fn main() {
    let content = crate::input::get_input().unwrap();

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

    let zeroes: Vec<&str> = valves.iter().filter_map(|(&k, v)| {
        if v.flow_rate == 0 && k != "AA" {
            Some(k)
        } else {
            None
        }
    }).collect();

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
    println!("{curr_path}");
    if valves.len() == curr_path.open_valves.len() {
        return curr_path.total_flow;
    }

    let mut keys: Vec<&&str> = valves.keys().collect();
    keys.sort();

    let filtered_paths = keys.into_iter()
        .filter(|k| !curr_path.open_valves.contains(k))
        .map(|k| get_shortest_path(curr_path.curr_valve, k, valves))
        .filter(|(_, dist)| dist + 1 <= curr_path.remaining_mins)
        .filter(|(path, _)| path.iter().rev().skip(1).all(|v| curr_path.open_valves.contains(v)))
        .map(|(path, dist)| (&valves[path.last().unwrap()], dist))
        .collect::<Vec<_>>();

    let mut max_flow = curr_path.total_flow;

    // Only go to valves that are directly reachable
    // without walking past a closed valve
    for (v, dist) in filtered_paths {
        let mut subpath = curr_path.clone();
        subpath.visited.push(v.key);
        subpath.open_valves.push(v.key);
        subpath.remaining_mins -= dist + 1;
        subpath.curr_valve = v.key;
        subpath.total_flow += v.flow_rate * subpath.remaining_mins;

        let subpath_flow = path_search(subpath, valves);
        if subpath_flow > max_flow {
            max_flow = subpath_flow;
        }
    }

    max_flow
}

fn get_shortest_path<'a>(from: &str, to: &str, valves: &'a HashMap<&'a str, Valve>) -> (Vec<&'a str>, usize) {
    let mut paths: HashMap<&str, (Vec<&str>, usize)> = valves.keys().map(|&k| (k, (vec![], usize::MAX))).collect();
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

            if curr_dist + dist < paths[k].1 {
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
        write!(f, "{}: {}", self.total_flow, self.visited.join(", "))
    }
}
