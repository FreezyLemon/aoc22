use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let inv_list = std::fs::read_to_string(input_file)?;
        let elf_inv = inv_list.split("\n\n");

        let mut sums_per_inv = elf_inv
            .map(|str| {
                str
                    .split("\n")
                    .map(|line| line.parse::<usize>().expect("can parse as integer"))
                    .sum::<usize>()
            })
            .collect::<Vec<_>>();

        sums_per_inv.sort();
        sums_per_inv.reverse(); // inefficient but i'm lazy

        let result = sums_per_inv[0] + sums_per_inv[1] + sums_per_inv[2];
        println!("result: {result}");
    } else {
        eprintln!("needs one argument");
        exit(-1);
    }

    Ok(())
}
