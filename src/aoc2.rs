use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_reports(file_path: &String) -> Vec<Vec<i32>> {
    let path = Path::new(file_path);
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let file = File::open(file_path).expect("Couldn't open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Couldn't read line");
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        reports.push(row);
    }

    reports
}

fn check_monotonicity_decreasing(report: &[i32]) -> bool {
    report.windows(2)
        .all(
            |pair|
                pair[0] > pair[1] && (pair[0] - pair[1]).abs() <= 3
        )
}

fn check_monotonicity_increasing(report: &[i32]) -> bool {
    report.windows(2)
        .all(
            |pair|
                pair[0] < pair[1] && (pair[0] - pair[1]).abs() <= 3
        )
}

fn check_monotonicity(report: &[i32]) -> bool {
    check_monotonicity_decreasing(report) || check_monotonicity_increasing(report)
}

fn calculate_safe_reports(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports: i32 = 0;
    reports.iter().for_each(|x| {
        if check_monotonicity(x) {
            safe_reports = safe_reports + 1;
        }
    });
    safe_reports
}

fn calculate_safe_reports_with_dampening(reports: &[Vec<i32>]) -> i32 {
    let mut safe_reports: i32 = 0;

    for report in reports {
        if check_monotonicity(report) {
            safe_reports += 1;
        } else {
            for skip in 0..report.len() {
                let mut reduced_report = report.to_vec();
                reduced_report.remove(skip);
                if check_monotonicity(&reduced_report) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    safe_reports
}

pub fn run() {
    let reports = read_reports(&"data/aoc2.txt".to_string());
    println!("Solution for Problem 2 : ");
    println!("Part 1: {}", calculate_safe_reports(&reports));
    println!("Part 2: {}", calculate_safe_reports_with_dampening(&reports));
    println!("***************");
}