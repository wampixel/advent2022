use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &'static str = "datas/day5.txt";
const STARTCRATES: [&'static str; 9] = [
    "NBDTVGZJ", "SRMDWPF", "VCRSZ", "RTJZPHG", "TCJNDZQF",
    "NVPWGSFM", "GCVBPQ", "ZBPN", "WPJ"
];

pub fn resolve() {
    let results: Vec<String> = read(FILENAME);
    println!("Day five, first star: {}", results[0]);
    println!("Day five, second star: {}", results[1]);
}

fn read(filename: &str) -> Vec<String> {
    let mut crates1: Vec<Vec<char>> = Vec::new();
    let mut crates2: Vec<Vec<char>> = Vec::new();
    STARTCRATES.iter().for_each(|s| crates1.push(s.chars().collect()));
    STARTCRATES.iter().for_each(|s| crates2.push(s.chars().collect()));
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let splited: Vec<&str> = line.split(" ").collect();
        let mut tmp: Vec<char> = vec![];

        for _ in 0..splited[1].parse::<usize>().unwrap(){
            match crates1[splited[3].parse::<usize>().unwrap() - 1].pop() {
                Some(c) => crates1[splited[5].parse::<usize>().unwrap() - 1].push(c),
                None => {}
            }
            match crates2[splited[3].parse::<usize>().unwrap() - 1].pop() {
                Some(c) => tmp.push(c),
                None => {}
            }
        }
        tmp.reverse();
        for c in tmp.iter() {
            crates2[splited[5].parse::<usize>().unwrap() - 1].push(*c);
        }
    }
    let result1: String = crates1.iter().map(|c| c.last().unwrap()).collect();
    let result2: String = crates2.iter().map(|c| c.last().unwrap()).collect();

    vec![result1, result2]
}