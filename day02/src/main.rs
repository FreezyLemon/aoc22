use get_input::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guide = get_input()?;
    
    let total_score = guide
        .split("\n")
        .map(|line| line.split_once(' ').expect("line has space"))
        .map(|(a, b)| (Choice::from(a), GameResult::from(b)))
        .map(|(a, b)| a.play(b))
        .sum::<usize>();

    println!("total score: {total_score}");

    Ok(())
}

#[derive(Clone, PartialOrd, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone)]
enum GameResult {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

impl Choice {
    fn play(self, result: GameResult) -> usize {
        let result_score = result.clone() as usize * 3;
        let mut other_choice = self as isize + result as isize - 1;
        if other_choice == 0 {
            other_choice = 3;
        } else if other_choice > 3 {
            other_choice %= 3;
        }

        result_score + other_choice as usize
        // (((self as isize + result as isize - 2) % 3) + 1) as usize
    }
}

impl From<&str> for Choice {
    fn from(s: &str) -> Self {
        if s.len() != 1 {
            panic!("must be of length 1");
        }

        let c = s.chars().next().unwrap();

        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unimplemented!(),
        }
    }
}

impl From<&str> for GameResult {
    fn from(s: &str) -> Self {
        if s.len() != 1 {
            panic!("must be of length 1");
        }

        let c = s.chars().next().unwrap();
        match c {
            'X' => GameResult::Loss,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => unimplemented!(),
        }
    }
}
