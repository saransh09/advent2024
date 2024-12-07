use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file(file_path: String) -> (Vec<i32>, Vec<i32>) {
    let path = Path::new(&file_path);

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    let file = File::open(&path).expect("File not found");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let (Some(x), Some(y)) = (parts.get(0), parts.get(1)) {
            if let (Ok(x), Ok(y)) = (x.parse::<i32>(), y.parse::<i32>()) {
                col1.push(x);
                col2.push(y);
            }
        }
    }

    (col1, col2)
}

fn return_dist(l1: &Vec<i32>, l2: &Vec<i32>) -> i32 {
    let mut sorted_l1 = l1.clone();
    let mut sorted_l2 = l2.clone();

    sorted_l1.sort();
    sorted_l2.sort();

    let mut dist = 0;
    for (i1, i2) in sorted_l1.iter().zip(sorted_l2.iter()) {
        dist += (i1 - i2).abs();
    }
    dist
}

fn similarity_score(l1: &Vec<i32>, l2: &Vec<i32>) -> i32{
    let mut c1 = HashMap::new();
    let mut c2 = HashMap::new();

    for num in l1 {
        *c1.entry(num).or_insert(0) += 1;
    }
    for num in l2 {
        *c2.entry(num).or_insert(0) += 1;
    }

    let mut similarity: i32 = 0;

    for (&k, &v) in &c1 {
        let curr = k * v;
        if let Some(&count_in_c2) = c2.get(&k) {
            similarity += curr * count_in_c2;
        }
    }

    similarity
}

pub fn run() {
    let file_path: String = "data/aoc1.txt".to_string();
    let (col1, col2) = read_file(file_path);
    println!("Solution for Problem 1 : ");
    println!("Part 1 : {}", return_dist(&col1, &col2));
    println!("Part 2 : {}", similarity_score(&col1, &col2));
    println!("***************");
}