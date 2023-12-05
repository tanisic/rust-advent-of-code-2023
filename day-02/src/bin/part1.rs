use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let first_part = &line[0..line.find(':').unwrap()];
        let id = first_part
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let second_part = &line[line.find(':').unwrap() + 1..];
        let second_vec = second_part.split([';', ',']).collect::<Vec<_>>();
        let mut prepare: (i32, i32, i32) = (0, 0, 0); // r,g,b
        for item in second_vec {
            let mut item_iter = item.trim().split_whitespace();
            let count = item_iter.next().unwrap().parse::<i32>().unwrap();
            // can be red, green or blue
            let color = item_iter.next().unwrap();
            if color.eq("red") {
                prepare.0 = prepare.0.max(count);
            } else if color.eq("green") {
                prepare.1 = prepare.1.max(count);
            } else if color.eq("blue") {
                prepare.2 = prepare.2.max(count);
            }
        }
        Self {
            blue: prepare.2,
            green: prepare.1,
            red: prepare.0,
            id,
        }
    }
}

fn main() {
    let path = Path::new("input2.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut games: Vec<Game> = vec![];
    for line in reader.lines() {
        games.push(Game::new(&line.unwrap()));
    }
    let mut ids_sum: i32 = 0;
    for game in games {
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            ids_sum += game.id;
        }
    }
    println!("Total id sum is: {}", ids_sum);
}
