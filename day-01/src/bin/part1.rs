use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("input1.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in reader.lines() {
        let mut digits: Vec<i32> = vec![];
        for sym in line.unwrap().chars() {
            if sym.is_numeric() {
                digits.push(sym.to_digit(10).unwrap().try_into().unwrap())
            }
        }
        sum += get_final_number(&digits)
    }
    println!("Total sum is: {}", sum)
}

fn get_final_number(v: &Vec<i32>) -> i32 {
    match v.len() {
        0 => 0,
        1 => {
            let digit = *v.to_owned().first().unwrap();
            digit * 10 + digit
        }
        _ => {
            let first_digit = *v.to_owned().first().unwrap();
            let last_digit = *v.to_owned().last().unwrap();
            first_digit * 10 + last_digit
        }
    }
}
