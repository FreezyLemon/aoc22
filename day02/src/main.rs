use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);

    if let Some(input_file) = args.next() {
        let guide = std::fs::read_to_string(input_file)?;
        
        let total_score = guide
            .split("\n")
            .map(|line| line.split_once(' ').expect("line has space"))
            .map(|(a, b)| (Choice::from(a), Choice::from(b)))
            .map(|(a, b)| b.play(a))
            .sum::<usize>();

        println!("total score: {total_score}");
    } else {
        eprintln!("needs one argument");
        exit(-1);
    }

    Ok(())
}

#[derive(Clone, PartialOrd, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn play(self, other: Choice) -> usize {
        let delta = self.clone() as isize - other as isize;
        let result = if delta == 0 {
            3 // draw
        } else if (delta + 3) % 3 == 1 {
            6 // win
        } else {
            0 // loss
        };

        result + self as usize
    }
}

impl From<&str> for Choice {
    fn from(s: &str) -> Self {
        if s.len() != 1 {
            panic!("must be of length 1");
        }

        let c = s.chars().next().unwrap();

        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unimplemented!(),
        }
    }
}
