use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let input = read_input();

    let calib_val = find_calibration(&input);
    let true_calib_val = find_true_calibration(&input);

    println!("Calibration value: {}", calib_val);
    println!("True calibration value: {}",true_calib_val);
}

fn read_input() -> Vec<String> {
    let mut s = String::new();
    let _ = File::open("input01.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    s
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn find_calibration(input: &Vec<String>) -> i64 {
    let mut calibration: i64 = 0;
    for s in input {
        let digits = s
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<Vec<char>>();
        if digits.is_empty() { continue; }
        calibration += (10 * digits.first().unwrap().to_digit(10).unwrap()
                        + digits.last().unwrap().to_digit(10).unwrap()) as i64;
    }
    calibration
}

fn find_true_calibration(input: &Vec<String>) -> i64 {
    let mut calibration: i64 = 0;
    let re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    for s in input {
        let mut i = 0;
        let mut last = "";
        while i < s.len() {
            if let Some(mtch) = re.find(&s[i..]) {
                last = mtch.as_str();
                if i == 0 { calibration += 10 * to_i64(last); }
                i += mtch.start() + 1;
            } else { break; }
        }
        if last != "" { calibration += to_i64(last); }
    }
    calibration
}

fn to_i64(s: &str) -> i64 {
    if s.len() == 1 { return s.parse::<i64>().unwrap() }
    match s {
        "zero"  => 0,
        "one"   => 1,
        "two"   => 2,
        "three" => 3,
        "four"  => 4,
        "five"  => 5,
        "six"   => 6,
        "seven" => 7,
        "eight" => 8,
        "nine"  => 9,
        _       => unreachable!(),
    }
}


