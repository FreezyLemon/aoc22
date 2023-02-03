use get_input::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pairs = get_input()?;

    let a = pairs
        .split('\n')
        .map(|line| line.split_once(',').expect("has a comma"))
        .filter(|(l, r)| {
            let (l_0, l_1) = l.split_once('-').expect("has -");
            let l_s = l_0.parse::<usize>().unwrap();
            let l_e = l_1.parse::<usize>().unwrap();

            let (r_0, r_1) = r.split_once('-').expect("has -");
            let r_s = r_0.parse::<usize>().unwrap();
            let r_e = r_1.parse::<usize>().unwrap();

            let mut l = l_s..=l_e;
            let r = r_s..=r_e;
            l.any(|i| r.contains(&i))
        })
        .count();

    println!("result {a}");

    Ok(())
}
