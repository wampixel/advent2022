use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Enums
enum Results {
    Win = 6,
    Draw = 3,
    Lose = 0
}

#[derive(Copy, Clone, PartialEq)]
enum Possibility {
    None,
    Rock,
    Paper,
    Scissors,
}

// implementations
impl Possibility {
    pub fn lose(self) -> Possibility {
        match self {
            Self::None => Self::None,
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock
        }
    }

    pub fn win(self) -> Possibility {
        match self {
            Self::None => Self::None,
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper
        }
    }

    pub fn score(self, oponent: Possibility) -> usize {
        if self == oponent.lose() {
            return self as usize + Results::Win as usize;
        } else if self == oponent.win() {
            return self as usize + Results::Lose as usize;
        }
        self as usize + Results::Draw as usize
    }
}

//Consts
const FILENAME: &'static str = "datas/day2.txt";

pub fn resolve() {
    let results: Vec<usize> = read(FILENAME);
    println!("Day rwo, first star: {}", results[0]);
    println!("Day rwo, second star: {}", results[1]);
}

fn read(filename: &str) -> Vec<usize> {
    let mut score1: usize = 0;
    let mut score2: usize = 0;
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let round: Vec<&str> = line.split(" ").collect();
        let oponent: Possibility = match round[0] {
            "A" => Possibility::Rock,
            "B" => Possibility::Paper,
            "C" => Possibility::Scissors,
            _ => Possibility::None // Impossible case
        };

        match round[1] {
            "X" => {
                score1 += Possibility::Rock.score(oponent);
                score2 += Results::Lose as usize + oponent.win() as usize
            }
            "Y" => {
                score1 += Possibility::Paper.score(oponent);
                score2 += Results::Draw as usize + oponent as usize
            }
            "Z" => {
                score1 += Possibility::Scissors.score(oponent);
                score2 += Results::Win as usize + oponent.lose() as usize
            }
            _ => {} // Imposible Case
        };
    }
    vec![score1, score2]
}