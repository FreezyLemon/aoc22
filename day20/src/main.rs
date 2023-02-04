mod input;

fn main() {
    let content = crate::input::get_input().unwrap();

    let mut encrypted: Vec<(usize, i64)> = content
        .split('\n')
        .map(|line| line.parse().unwrap())
        .map(|v: i64| v * 811589153)
        .enumerate()
        .collect();

    let l = encrypted.len();

    // mix
    for _ in 0..10 {
        for idx in 0..l {
            let idx = index_of(&encrypted, |(oi, _)| *oi == idx).unwrap();
            let elem = encrypted.remove(idx);
            let offset = elem.1.rem_euclid(l as i64 - 1);
    
            let new_idx = (idx as i64 + offset) as usize % (l - 1);
    
            encrypted.insert(new_idx, elem);
        }
    }

    let zero_idx = index_of(&encrypted, |(_, val)| *val == 0).unwrap();
    let first_idx = (zero_idx + 1000) % l;
    let second_idx = (first_idx + 1000) % l;
    let third_idx = (second_idx + 1000) % l;

    let result = encrypted[first_idx].1 + encrypted[second_idx].1 + encrypted[third_idx].1;

    println!("result: {result}");
}

fn index_of<T, F: Fn(&T) -> bool>(vec: &Vec<T>, predicate: F) -> Option<usize> {
    let mut idx = 0;
    
    for elem in vec {
        if predicate(elem) {
            return Some(idx);
        }

        idx += 1;
    }

    None
}
