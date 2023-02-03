use core::panic;
use std::collections::HashMap;

use get_input::*;

fn main() {
    let input = get_input().unwrap();

    // assumption: monkey names are 4 chars long
    let monkeys: HashMap<&str, Job> = input
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

    let root = monkeys.get("root").expect("root monkey exists");
    let result = resolve_job(root, &monkeys);

    println!("result: {result}");
}

fn resolve_job(job: &Job, monkeys: &HashMap<&str, Job>) -> i64 {
    match job {
        Job::Number(i) => *i,
        Job::Calculate(l, r, op) => {
            let l = monkeys.get(l).unwrap();
            let r = monkeys.get(r).unwrap();

            let l = resolve_job(l, monkeys);
            let r = resolve_job(r, monkeys);

            match op {
                Operation::Add => l + r,
                Operation::Multiply => l * r,
                Operation::Subtract => l - r,
                Operation::Divide => l / r,
            }
        }
    }
}

enum Job<'a> {
    Number(i64),
    Calculate(&'a str, &'a str, Operation),
}

enum Operation {
    Add,
    Multiply,
    Subtract,
    Divide,
}
