use core::panic;
use std::collections::{HashMap, HashSet};

use aoc2022::get_input;

const HUMN: &str = "humn";

fn main() {
    let input = get_input().unwrap();

    // assumption: monkey names are 4 chars long
    let mut monkeys: HashMap<&str, Job> = input
        .split('\n')
        .map(|line| {
            let (name, job) = line.split_at(4);
            let job = &job[2..]; // skip ': '

            let job = if !job.contains(' ') {
                Job::Number(job.parse().unwrap())
            } else {
                let left_m = &job[..4];
                let right_m = &job[7..];

                let operation = match job.chars().nth(5).expect("long enough") {
                    '+' => Operation::Add,
                    '*' => Operation::Multiply,
                    '-' => Operation::Subtract,
                    '/' => Operation::Divide,
                    _ => panic!("expected math operator"),
                };

                Job::Calculate(left_m, right_m, operation)
            };

            (name, job)
        })
        .collect();

    // find all Jobs that can be calculated without "humn"
    let mut to_simplify = vec![];
    for (m, j) in &monkeys {
        if let Job::Number(_) = j {
            continue;
        }

        match simplify_job(j, &monkeys) {
            Some(x) => to_simplify.push((*m, x)),
            None => continue,
        }
    }

    // replace all "simplifiable" Jobs with just a Job::Number(x)
    // also collect all Jobs that were referenced by the old calculation
    let mut to_check = HashSet::new();
    for (m, v) in to_simplify {
        let old = monkeys.insert(m, Job::Number(v)).unwrap();

        if let Job::Calculate(l, r, _) = old {
            to_check.insert(l);
            to_check.insert(r);
        }
    }

    // if possible, remove all old Jobs that are no longer referenced
    for c in to_check {
        // check if we can remove this key/value pair from the HashMap
        let is_unreferenced = monkeys
            .iter()
            .filter_map(|(_, j)| {
                match j {
                    Job::Number(_) => None,
                    Job::Calculate(l, r, _) => Some((l, r)),
                }
            })
            .all(|(l, r)| { *l != c && *r != c });

        if is_unreferenced {
            monkeys.remove(c);
        }
    }

    let humn = monkeys.get(HUMN).expect("'humn' exists");
    let start_val = humn.clone().unwrap_number();
    let root = monkeys.get("root").expect("root monkey exists").clone();
    let (l, r, _) = root.unwrap_calc();
    let l = monkeys.get(l).unwrap().clone();
    let r = monkeys.get(r).unwrap().clone();
    let start_res = resolve_job(&l, &monkeys);

    // seems to always be the case
    let target = r.unwrap_number();
    let increment = 100_000_000;
    let test_val = start_val + increment;

    monkeys.insert(HUMN, Job::Number(test_val));
    let test_res = resolve_job(&l, &monkeys);

    // try to extrapolate
    let delta = (target - test_res).abs() - (target - start_res).abs();
    let m = delta as f64 / increment as f64;

    let interpolated = (target - start_res) as f64 / m;
    let interpolated = interpolated.round() as i64;

    monkeys.insert(HUMN, Job::Number(interpolated));
    let maybe = resolve_job(&l, &monkeys);
    println!("maybe: {maybe}, target: {target}");
    let dist = target - maybe;
    println!("m: {m}, dist from target: {dist}");

    let steps = 2 * (dist as f64 / m) as i64;
    println!("approx steps: {steps}");
    let start = if steps > 0 {
        interpolated
    } else {
        interpolated + steps
    };

    let end = if steps > 0 {
        interpolated + steps
    } else {
        interpolated
    };

    for v in start..end {
        monkeys.insert(HUMN, Job::Number(v as i64));
        let res = resolve_job(&l, &monkeys);

        if res == target {
            println!("result: {v}");
            break;
        }
    }
}

fn simplify_job(job: &Job, monkeys: &HashMap<&str, Job>) -> Option<i64> {
    match job {
        Job::Number(i) => Some(*i),
        Job::Calculate(l, r, op) => {
            if *l == HUMN || *r == HUMN {
                return None;
            }
            
            let l = monkeys.get(l).unwrap();
            let r = monkeys.get(r).unwrap();

            let l = simplify_job(l, monkeys);
            let r = simplify_job(r, monkeys);

            if l.is_none() || r.is_none() {
                return None;
            }

            Some(op.calc(l.unwrap(), r.unwrap()))
        }
    }
}

fn resolve_job(job: &Job, monkeys: &HashMap<&str, Job>) -> i64 {
    match job {
        Job::Number(i) => *i,
        Job::Calculate(l, r, op) => {
            let l = monkeys.get(l).unwrap();
            let r = monkeys.get(r).unwrap();

            let l = resolve_job(l, monkeys);
            let r = resolve_job(r, monkeys);

            op.calc(l, r)
        }
    }
}

#[derive(Clone)]
enum Job<'a> {
    Number(i64),
    Calculate(&'a str, &'a str, Operation),
}

impl<'a> Job<'a> {
    fn unwrap_number(self) -> i64 {
        match self {
            Job::Number(x) => x,
            Job::Calculate(_, _, _) => panic!("not a number"),
        }
    }

    fn unwrap_calc(self) -> (&'a str, &'a str, Operation) {
        match self {
            Job::Number(_) => panic!("not a calculation"),
            Job::Calculate(l, r, op) => (l, r, op),
        }
    }
}

#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl Operation {
    fn calc(&self, l: i64, r: i64) -> i64 {
        match self {
            Operation::Add => l + r,
            Operation::Multiply => l * r,
            Operation::Subtract => l - r,
            Operation::Divide => l / r,
        }
    }
}
