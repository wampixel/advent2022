use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &'static str = "datas/day1.txt";

pub fn resolve() {
    let v: Vec<usize> = read(FILENAME);
    println!("Day one first star: {}", v[0]);
    println!("Day one, second star: {}", (0..3).fold(0, |a, b| a + v[b]));
}

fn read(filename: &str) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut tmp: usize = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(tmp);
            tmp = 0;
        } else {
            tmp += line.parse::<usize>().unwrap();
        }
    }
    result.sort();
    result.reverse();
    result
}