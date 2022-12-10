use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &'static str = "datas/day6.txt";

pub fn resolve() {
    let results: Vec<usize> = read(FILENAME);
    println!("Day six, first star: {}", results[0]);
    println!("Day six, second star: {}", results[1]);

}

pub fn read(filename: &str) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut buf: Vec<char> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            if buf.len() == 4 && result.len() < 1{
                result.push(i);
            } 
            if buf.len() == 14 {
                result.push(i);
                break;
            }
            if buf.contains(&c) {
                let idx = buf.iter().position(|x| x == &c).unwrap();
                buf.drain(..idx+1);
            }
            buf.push(c);
        }
    }

    result
}