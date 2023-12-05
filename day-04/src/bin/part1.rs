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
    fn get_points(&self) -> i32 {
        let mut mutual_count: i32 = 0;
        for num in self.numbers.clone() {
            if self.winning_numbers.iter().any(|wn| *wn == num) {
                if mutual_count > 1 {
                    mutual_count *= 2;
                } else {
                    mutual_count += 1;
                }
            }
        }
        mutual_count
    }
}

fn main() {
    let path = Path::new("input3.txt");
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut total: i32 = 0;
    for line in reader.lines() {
        let c = Card::new(&line.unwrap());
        let points = c.get_points();
        total += points;
    }
    println!("Total: {}", total);
}
