use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn new(string: &str) -> Self {
        let winning_part = &string[string.find(':').unwrap() + 1..string.find('|').unwrap()]
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let numbers_part = &string[string.find('|').unwrap() + 1..]
            .trim()
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        Self {
            numbers: numbers_part.clone(),
            winning_numbers: winning_part.clone(),
        }
    }
    fn get_matches_count(&self) -> usize {
        let mut mutual_count: usize = 0;
        for num in self.numbers.clone() {
            if self.winning_numbers.iter().any(|wn| *wn == num) {
                mutual_count += 1;
            }
        }
        mutual_count
    }
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();
    let cnt_reader: BufReader<File> = BufReader::new(file);
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut idx: usize = 0;
    let mut total_arr: Vec<i32> = vec![1; cnt_reader.lines().count()];
    for line in reader.lines() {
        let str = line.unwrap();
        let c = Card::new(&str);
        let matches = c.get_matches_count();
        if matches > 0 {
            for i in 1..=matches {
                total_arr[idx + i] += total_arr[idx];
            }
        }
        idx += 1;
    }
    let mut total: i32 = 0;
    for card in &total_arr {
        total += card;
    }
    println!("{:?}\nTotal {}", total_arr, total);
}
