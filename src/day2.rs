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

#[derive(Copy, Clone)]
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
        self as usize + 
        match (self, oponent) {
            (Self::Rock, Self::Scissors) => Results::Win,
            (Self::Paper, Self::Rock) => Results::Win,
            (Self::Scissors, Self::Paper) => Results::Win,
            (Self::Rock, Self::Paper) => Results::Lose,
            (Self::Paper, Self::Scissors) => Results::Lose,
            (Self::Scissors, Self::Rock) => Results::Lose,
            _ => Results::Draw
        } as usize 
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

        score1 += match round[1] {
            "X" => Possibility::Rock.score(oponent),
            "Y" => Possibility::Paper.score(oponent),
            "Z" => Possibility::Scissors.score(oponent),
            _ => Possibility::None.score(oponent) // Imposible Case
        };

        score2 += match round[1] {
            "X" => {Results::Lose as usize + oponent.win() as usize}
            "Y" => {Results::Draw as usize + oponent as usize}
            "Z" => {Results::Win as usize  + oponent.lose() as usize}
            _ => 0
        };
    }
    vec![score1, score2]
}
