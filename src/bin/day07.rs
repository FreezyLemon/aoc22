use std::collections::HashMap;

use aoc2022::get_input;

const DIR_DELIM: &str = "/";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = get_input()?;

    let mut file_sizes = HashMap::new();
    let mut curr_dir = String::from("/");
    for cmd in content.split("$ ").skip(1) {
        match &cmd[..2] {
            "ls" => {
                let total_size: usize = cmd
                    .split('\n')
                    .skip(1)
                    .filter(|&l| l.len() > 0)
                    .filter_map(|l| {
                        let (size, _name) = l.split_once(' ').unwrap();
                        size.parse::<usize>().ok()
                    })
                    .sum();

                file_sizes.insert(curr_dir.to_string(), total_size);
            },
            "cd" => match cmd.split_whitespace().skip(1).next().unwrap() {
                "/" => curr_dir = "/".to_string(),
                ".." => {
                    let idx = curr_dir.rfind(DIR_DELIM).unwrap();
                    curr_dir.truncate(idx);
                },
                dir => {
                    if !curr_dir.ends_with(DIR_DELIM) {
                        curr_dir += DIR_DELIM;
                    }
                    curr_dir += dir;
                },
            },
            _ => unimplemented!(),
        }
    }

    let mut dir_sizes = HashMap::new();
    let all_dirs = file_sizes.keys().into_iter().collect::<Vec<_>>();
    for &dir in all_dirs.iter() {
        let subdir_file_sizes: usize = all_dirs
            .iter()
            .filter(|d| d.starts_with(dir))
            .map(|&d| file_sizes.get(d).unwrap())
            .sum();
        
        dir_sizes.insert(dir, subdir_file_sizes);
    }

    let mut dir_sizes = dir_sizes.into_values().collect::<Vec<_>>();
    dir_sizes.sort();

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    
    let free_space = total_space - dir_sizes.last().unwrap();
    let space_to_delete = needed_space - free_space;
    let result = dir_sizes.into_iter().find(|&s| s >= space_to_delete).unwrap();
    println!("{result}");

    Ok(())
}
