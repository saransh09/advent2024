use regex::Regex;
use std::fs;

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Can't read file")
}

fn uncorrupt(expression: &str) -> i32 {
    let regex: &str = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(regex).expect("Invalid regexp");

    re.captures_iter(expression)
        .map(|caps| {
            let num1: i32 = caps[1].parse().unwrap_or(0); // Safely parse num1
            let num2: i32 = caps[2].parse().unwrap_or(0); // Safely parse num2
            num1 * num2
        })
        .sum()
}

fn uncorrupt_extended(sample: &str) -> i32 {
    let regex = r"(mul\([1-9][0-9]{0,2},[1-9][0-9]{0,2}\)|don't|\bdo\b|do)";
    let re = Regex::new(regex).expect("Invalid regex");

    let parse_mul = |occ: &str| -> Option<i32> {
        let num1_start = occ.find('(')? + 1;
        let num1_end = occ.find(',')?;
        let num2_start = occ.rfind(',')? + 1;
        let num2_end = occ.rfind(')')?;
        let num1: i32 = occ[num1_start..num1_end].parse().ok()?;
        let num2: i32 = occ[num2_start..num2_end].parse().ok()?;
        Some(num1 * num2)
    };

    let mut sol = 0;
    let mut disabled = false;

    re.find_iter(sample).for_each(|mat| {
        let occ = mat.as_str();
        match occ {
            "do" => disabled = false,
            "don't" => disabled = true,
            _ if !disabled => {
                if let Some(product) = parse_mul(occ) {
                    sol += product;
                }
            }
            _ => {}
        }
    });

    sol
}

pub fn run() {
    let expression = read_file("data/aoc3.txt");
    println!("Solution for Problem 3 : ");
    println!("Part 1 : {}", uncorrupt(&expression));
    println!("Part 2 : {}", uncorrupt_extended(&expression));
    println!("***************");
}