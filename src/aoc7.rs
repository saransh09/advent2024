use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::time::Instant;

type Equation = (i64, Vec<i64>);

fn read_equations(filepath: &str) -> Vec<Equation> {
    let mut equations: Vec<Equation> = Vec::new();
    let file = File::open(filepath).expect("File not found");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Problem reading file");
        let mut parts = line.split_whitespace();
        let val: i64 = parts.next().unwrap().trim_end_matches(':').parse().unwrap();
        let equation: Vec<i64> = parts.map(|x| x.parse().unwrap()).collect();
        equations.push((val, equation));
    }

    equations
}

fn place_operations_helper_recursion(equations: &[i64], curr_ind: usize, sol: i64, sum: i64) -> bool {
    if sum > sol {
        return false;
    }

    if curr_ind == equations.len() {
        return sum == sol;
    }

    place_operations_helper_recursion(equations, curr_ind + 1, sol, sum + equations[curr_ind])
        || place_operations_helper_recursion(equations, curr_ind + 1, sol, sum * equations[curr_ind])
}


fn place_operations_helper_dp_top_down(equations: &[i64], curr_ind: usize, sol: i64, sum: i64, memo: &mut HashMap<(usize, i64), bool>) -> bool {
    if sum > sol {
        return false;
    }

    if curr_ind == equations.len() {
        return sum == sol;
    }

    let key = (curr_ind, sum);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let add_result = place_operations_helper_dp_top_down(
        equations,
        curr_ind + 1,
        sol,
        sum + equations[curr_ind],
        memo,
    );

    let mul_result = place_operations_helper_dp_top_down(
        equations,
        curr_ind + 1,
        sol,
        sum * equations[curr_ind],
        memo,
    );

    let result = add_result || mul_result;
    memo.insert(key, result);
    result
}

fn place_operations_helper_dp_bottom_up(equations: &[i64], sol: i64) -> bool {
    if equations.is_empty() {
        return false;
    }

    let mut current: HashSet<i64> = HashSet::new();
    current.insert(equations[0]);

    for &num in &equations[1..] {
        let mut next_set = HashSet::new();
        for &s in &current {
            next_set.insert(s + num);
            next_set.insert(s * num);
        }
        current = next_set;
    }

    current.contains(&sol)
}

fn run_first_part(filepath: &str) {
    let all_equations = read_equations(filepath);
    let mut accum_val = 0;

    let start = Instant::now();
    for (val, equations) in &all_equations {
        if place_operations_helper_recursion(equations, 1, *val, equations[0]) {
            accum_val += val;
        }
    }
    println!("Part 1 took {:?} with naive recursion based approach, solution : {}", start.elapsed(), accum_val);

    accum_val = 0;
    let start = Instant::now();
    for (val, equations) in &all_equations {
        let mut dp_top_down_memo: HashMap<(usize, i64), bool> = HashMap::new();
        if place_operations_helper_dp_top_down(equations, 1, *val, equations[0], &mut dp_top_down_memo) {
            accum_val += val;
        }
    }
    println!("Part 1 took {:?} with top down Dynamic Programming based approach, solution : {}", start.elapsed(), accum_val);

    accum_val = 0;
    let start = Instant::now();
    for (val, equations) in &all_equations {
        if place_operations_helper_dp_bottom_up(equations, *val) {
            accum_val += val;
        }
    }
    println!("Part 1 took {:?} with bottom up Dynamic Programming based approach, solution : {}", start.elapsed(), accum_val);
}

fn place_operations_extended_helper_recursion(equations: &[i64], curr_ind: usize, sol: i64, sum: i64) -> bool {
    if sum > sol {
        return false;
    }

    if curr_ind == equations.len() {
        return sol == sum;
    }

    place_operations_extended_helper_recursion(equations, curr_ind + 1, sol, sum + equations[curr_ind])
        || place_operations_extended_helper_recursion(equations, curr_ind + 1, sol, sum * equations[curr_ind])
        || place_operations_extended_helper_recursion(equations, curr_ind + 1, sol, format!("{}{}", sum, equations[curr_ind]).parse::<i64>().unwrap())
}

fn place_operations_extended_helper_dp_top_down(equations: &[i64], curr_ind: usize, sol: i64, sum: i64, memo: &mut HashMap<(usize, i64), bool>) -> bool {
    if sum > sol {
        return false;
    }

    if curr_ind == equations.len() {
        return sol == sum;
    }
    let key = (curr_ind, sum);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    let add_result = place_operations_extended_helper_dp_top_down(
        equations, curr_ind + 1, sol, sum + equations[curr_ind], memo);
    let mul_result = place_operations_extended_helper_dp_top_down(
        equations, curr_ind + 1, sol, sum * equations[curr_ind], memo);
    let con_result = place_operations_extended_helper_dp_top_down(
        equations, curr_ind + 1, sol, format!("{}{}", sum, equations[curr_ind]).parse::<i64>().unwrap(), memo);

    let result = add_result || mul_result || con_result;
    memo.insert(key, result);
    result
}

fn place_operations_extended_helper_dp_bottom_up(equations: &[i64], sol: i64) -> bool {
    if equations.is_empty() {
        return false;
    }

    let mut current: HashSet<i64> = HashSet::new();
    current.insert(equations[0]);

    for &num in &equations[1..] {
        let mut next_set = HashSet::new();
        for &s in &current {
            next_set.insert(s + num);
            next_set.insert(s * num);
            next_set.insert(format!("{}{}", s, num).parse::<i64>().unwrap());
        }
        current = next_set;
    }
    current.contains(&sol)
}

fn run_second_part(filepath: &str) {
    let all_equations = read_equations(filepath);
    let mut accum_val = 0;

    let start = Instant::now();
    for (val, equations) in &all_equations {
        if place_operations_extended_helper_recursion(equations, 1, *val, equations[0]) {
            accum_val += val;
        }
    }
    println!("Part 2 took {:?} with naive recursion based approach, solution : {}", start.elapsed(), accum_val);

    accum_val = 0;
    let start = Instant::now();
    for (val, equations) in &all_equations {
        let mut dp_top_down_memo: HashMap<(usize, i64), bool> = HashMap::new();
        if place_operations_extended_helper_dp_top_down(equations, 1, *val, equations[0], &mut dp_top_down_memo) {
            accum_val += val;
        }
    }
    println!("Part 2 took {:?} with top down Dynamic Programming based approach, solution : {}", start.elapsed(), accum_val);

    accum_val = 0;
    let start = Instant::now();
    for (val, equations) in &all_equations {
        if place_operations_extended_helper_dp_bottom_up(equations, *val) {
            accum_val += val;
        }
    }
    println!("Part 2 took {:?} with bottom up Dynamic Programming based approach, solution : {}", start.elapsed(), accum_val);
}

pub fn run() {
    println!("Solution for Problem 7 : ");
    run_first_part("data/aoc7.txt");
    run_second_part("data/aoc7.txt");
    println!("***************");
}