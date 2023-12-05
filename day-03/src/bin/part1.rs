use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    vec,
};

fn main() {
    let mut data = Box::new(generate_arr());
    let (arr, n) = data.borrow_mut();
    println!("{:?}, n is {}", arr, n);
    let part_numbers: Vec<i32> = vec![];
    for i in 0..*n {
        for j in 0..*n {
            let item = &arr[i * *n + j];
            if item.is_numeric() {
                let mut num = String::new();
                num.push(*item);
            }
        }
    }
}

fn generate_arr() -> (Vec<char>, usize) {
    let path = Path::new("summary.txt");
    let file = File::open(path).unwrap();
    let len_reader: BufReader<File> = BufReader::new(file);
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file.try_clone().unwrap());
    let n = len_reader.lines().count();
    let mut grid = vec!['.'; n * n];
    let mut i = 0;
    for line in reader.lines() {
        let mut j = 0;
        for c in line.unwrap().chars() {
            grid[i * n + j] = c.clone();
            j += 1;
        }
        i += 1;
    }
    (grid, n)
}

fn is_symbol(sym: &char) -> bool {
    *sym != '.' && !sym.is_numeric()
}

fn join_and_parse(substring: &[char]) -> i32 {
    let mut res = String::new();
    for c in substring {
        res.push(*c);
    }
    res.parse::<i32>().unwrap()
}
