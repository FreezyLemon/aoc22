use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let pairs = std::fs::read_to_string(input_file)?;
        
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
    } else {
        eprintln!("needs one argument");
        exit(-1);
    }

    Ok(())
}
