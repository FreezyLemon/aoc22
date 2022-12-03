use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let inv_list = std::fs::read_to_string(input_file)?;
        let elf_inv = inv_list.split("\n\n");

        let highest_calories = elf_inv
            .map(|str| {
                str
                    .split("\n")
                    .map(|line| line.parse::<usize>().expect("can parse as integer"))
                    .sum::<usize>()
            })
            .max()
            .expect("inventory contains entries");

        println!("result: {}", highest_calories);
    } else {
        eprintln!("needs one argument");
        exit(-1);
    }

    Ok(())
}
