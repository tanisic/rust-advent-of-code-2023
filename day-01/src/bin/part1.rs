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
        let res = line.unwrap();
        sum += get_line_result(&res[..]);
    }
    println!("Total sum is: {}", sum)
}

fn get_line_result(line: &str) -> i32 {
    let mut digits: Vec<i32> = vec![];
    for sym in line.chars() {
        if sym.is_digit(10) {
            let digit = sym.to_digit(10).unwrap().try_into().unwrap();
            digits.push(digit);
        }
    }
    match digits.len() {
        0 => 0,
        1 => {
            let digit = *digits.to_owned().first().unwrap();
            digit * 10 + digit
        }
        _ => {
            let first_digit = *digits.to_owned().first().unwrap();
            let last_digit = *digits.to_owned().last().unwrap();
            first_digit * 10 + last_digit
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_line_result;

    #[test]
    fn two_digits() {
        assert_eq!(get_line_result("pqr3stu8vwx"), 38);
    }
    #[test]
    fn multiple_digits() {
        assert_eq!(get_line_result("pqr3stu8vwx3abc"), 33);
    }
    #[test]
    fn multiple_single_digit() {
        assert_eq!(get_line_result("treb7uchet"), 77);
    }
}
