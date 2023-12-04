use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let path = Path::new("input1.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let res = line.unwrap();
        sum += get_line_result(&res[..]);
    }
    println!("Total sum is: {}", sum)
}

fn get_line_result(line: &str) -> u32 {
    let iter_result = (0..line.len()).filter_map(|idx| {
        let substring_number = &line[idx..];
        let result = if substring_number.starts_with("one") {
            Some(1)
        } else if substring_number.starts_with("two") {
            Some(2)
        } else if substring_number.starts_with("three") {
            Some(3)
        } else if substring_number.starts_with("four") {
            Some(4)
        } else if substring_number.starts_with("five") {
            Some(5)
        } else if substring_number.starts_with("six") {
            Some(6)
        } else if substring_number.starts_with("seven") {
            Some(7)
        } else if substring_number.starts_with("eight") {
            Some(8)
        } else if substring_number.starts_with("nine") {
            Some(9)
        } else {
            substring_number.chars().next().unwrap().to_digit(10)
        };
        result
    });

    let arr = iter_result.collect::<Vec<_>>();

    match arr.len() {
        0 => 0,
        1 => {
            let digit = *arr.to_owned().first().unwrap();
            digit * 10 + digit
        }
        _ => {
            let first_digit = *arr.to_owned().first().unwrap();
            let last_digit = *arr.to_owned().last().unwrap();
            first_digit * 10 + last_digit
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::get_line_result;

    #[test]
    fn day1_part2_2digit() {
        assert_eq!(get_line_result("pqr3stu8vwx"), 38);
    }
    #[test]
    fn day1_part2_multiple_digits() {
        assert_eq!(get_line_result("pqr3stu8vwx3abc"), 33);
    }
    #[test]
    fn day1_part2_multiple_single_digit() {
        assert_eq!(get_line_result("treb7uchet"), 77);
    }
}
