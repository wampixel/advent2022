use std::{
    ops::Range,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &'static str = "datas/day4.txt";

pub fn resolve() {
    let results: Vec<usize> = read(FILENAME);
    println!("Day four, first star: {}", results[0]);
    println!("Day four, second star: {}", results[1]);
}

fn read(filename: &str) -> Vec<usize> {
    let mut ctxc: usize = 0;
    let mut ctxo: usize = 0;
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let ranges: Vec<Vec<usize>> = line.split(",")
            .map(|s| s.split('-').collect())
            .map(|v: Vec<&str>| v.into_iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
            )
            .collect();
        if contains(ranges[0][0]..ranges[0][1]+1, &ranges[1]) 
            || contains(ranges[1][0]..ranges[1][1]+1, &ranges[0]) {
                ctxc += 1;
        }
        if overlap(ranges[0][0]..ranges[0][1], &ranges[1])
            || overlap(ranges[1][0]..ranges[1][1]+1, &ranges[0]) {
           ctxo += 1;
        }
    }
    vec![ctxc, ctxo]
}

fn contains(range1: Range<usize>, range2: &Vec<usize>) -> bool {
    range1.contains(&range2[0]) && range1.contains(&range2[1])
}

fn overlap(range1: Range<usize>, range2: &Vec<usize>) -> bool {
    range1.contains(&range2[0]) || range1.contains(&range2[1])
}
