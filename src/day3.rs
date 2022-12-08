use std::{
    fs::File,
    io::{BufRead, BufReader},
};
const FILENAME: &'static str = "datas/day3.txt";

pub fn resolve() {
    let results: Vec<usize> = read(FILENAME);
    println!("Day three, first star: {}", results[0]);
    println!("Day three, second star: {}", results[1]);
}
fn read(filename: &str) -> Vec<usize> {
    let mut score1: usize = 0;
    let file: File = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut groups: Vec<Vec<String>> = vec![];
    let mut group: Vec<String> = vec![];

    for (_, line) in reader.lines().enumerate() {
        let line: String = line.unwrap();
        let(first, last) = line.split_at(line.len()/2);
        for c in first.chars() { 
            if last.contains(c) {
                score1 += get_val(c);
                break;
            }
        }
        if group.len() < 3 {
            group.push(line)
        } else {
            groups.push(group);
            group = vec![line];
        }
    }
    // Add the last group
    let mut score2: usize = 0;
    groups.push(group);
    for group in groups {
        for c in group[0].chars() {
            if group[1].contains(c) && group[2].contains(c) {
                score2 += get_val(c);
                break;
            }
        }
    }
    vec![score1, score2]
}

fn get_val(c: char) -> usize {
    if c.is_uppercase() {
        return 27 + c as usize - 'A' as usize;
    }
    1 + c as usize - 'a' as usize
}