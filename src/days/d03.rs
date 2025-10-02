// https://adventofcode.com/2024/day/3

use regex::Regex;

const P1REGEX: &str = r#"mul\((\d{1,3}),(\d{1,3})\)"#;
const P2REGEX: &str = r#"(?:mul\((\d\d?\d?),(\d\d?\d?)\))|(?:(do\(\)|don\'t\(\)))"#;

pub fn solve(input: String) -> (String, String) {
    let p1regex = Regex::new(P1REGEX).expect("Unable to compile regex");
    let p1matches = p1regex.captures_iter(&input).collect::<Vec<_>>();

    let mut p1sum = 0;
    for cap in p1matches {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        p1sum += a * b;
    }

    let p2regex = Regex::new(P2REGEX).expect("Unable to compile regex");
    let p2matches = p2regex.captures_iter(&input).collect::<Vec<_>>();

    let mut p2sum = 0;
    let mut enabled = true;
    for cap in p2matches {
        if cap[0].contains("do") {
            if cap[3] == *"do()" {
                enabled = true;
            } else if cap[3] == *"don't()" {
                enabled = false;
            }
        } else {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            if enabled == true {
                p2sum += a * b;
            }
        }
    }


    (p1sum.to_string(), p2sum.to_string())
}
