use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> (Vec<i32>, Vec<i32>, Vec<Vec<i32>>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let mut print_lists: Vec<Vec<i32>> = Vec::new();

    let file = File::open(filename).expect("Couldn't read given file!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Couldn't read line");
        if let Some(index) = line.find('|') {
            let first = line[..index].trim().parse::<i32>().unwrap();
            let second = line[index + 1..].trim().parse::<i32>().unwrap();
            first_list.push(first);
            second_list.push(second);
        } else if line.contains(',') {
            let list: Vec<i32> = line
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            print_lists.push(list);
        }
    }

    (first_list, second_list, print_lists)
}

fn create_priority_map(first_list: &[i32], second_list: &[i32]) -> HashMap<i32, HashSet<i32>> {
    let mut priority_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (&f, &s) in first_list.iter().zip(second_list.iter()) {
        priority_map.entry(f).or_insert_with(HashSet::new).insert(s);
    }
    priority_map
}

fn check_list(print_list: &[i32], priority_map: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut to_be_printed = Vec::new();
    for &elem in print_list {
        for &tbp in &to_be_printed {
            if priority_map
                .get(&elem)
                .map_or(false, |deps| deps.contains(&tbp))
            {
                return false;
            }
        }
        to_be_printed.push(elem);
    }
    true
}

fn check_valid_lists_and_find_mid_sum(
    print_lists: &[Vec<i32>], priority_map: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut sum = 0;
    for print_list in print_lists {
        if check_list(print_list, priority_map) {
            sum += print_list[print_list.len() / 2];
        }
    }
    sum
}

fn find_invalid_lists(
    print_lists: &[Vec<i32>],
    priority_map: &HashMap<i32, HashSet<i32>>,
) -> Vec<Vec<i32>> {
    print_lists
        .iter()
        .filter(|list| !check_list(list, priority_map))
        .cloned()
        .collect()
}

fn create_flip_lists(first_list: &[i32], second_list: &[i32]) -> HashMap<i32, HashSet<i32>> {
    let mut priority_map_flip: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (&f, &s) in first_list.iter().zip(second_list.iter()) {
        priority_map_flip.entry(s).or_insert_with(HashSet::new).insert(f);
    }
    priority_map_flip
}

fn custom_sort(x: &i32, y: &i32, priority_map: &HashMap<i32, HashSet<i32>>) -> std::cmp::Ordering {
    if priority_map.get(x).map_or(false, |deps| deps.contains(y)) {
        std::cmp::Ordering::Greater
    } else if priority_map.get(y).map_or(false, |deps| deps.contains(x)) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

fn sort_incorrect_lists(
    invalid_lists: &[Vec<i32>],
    priority_map: &HashMap<i32, HashSet<i32>>,
) -> Vec<Vec<i32>> {
    invalid_lists
        .iter()
        .map(|list| {
            let mut owned_list = list.clone(); // Clone the list to get ownership
            owned_list.sort_by(|a, b| custom_sort(a, b, priority_map));
            owned_list
        })
        .collect()
}

fn find_mid_sum(incorrect_lists: &[Vec<i32>]) -> i32 {
    incorrect_lists.iter().map(|list| list[list.len() / 2]).sum()
}

pub fn run() {
    let (first_list, second_list, print_lists)
        = read_input("data/aoc5.txt");
    let priority_map = create_priority_map(&first_list, &second_list);

    println!("Solution for Problem 5 : ");

    let sum = check_valid_lists_and_find_mid_sum(&print_lists, &priority_map);
    println!("Part 1: {}", sum);

    let invalid_lists = find_invalid_lists(&print_lists, &priority_map);
    let flip_map = create_flip_lists(&first_list, &second_list);
    let sorted_lists = sort_incorrect_lists(&invalid_lists, &priority_map);

    println!("Part 2: {}", find_mid_sum(&sorted_lists));

    println!("***************");
}