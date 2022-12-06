mod input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inv_list = crate::input::get_input()?;
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

    Ok(())
}
