use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

static MOVE_DIR: Lazy<HashMap<char, (isize, isize)>> = Lazy::new(|| {
    vec![
        ('>', (0, 1)),
        ('v', (1, 0)),
        ('<', (0, -1)),
        ('^', (-1, 0)),
    ]
        .into_iter()
        .collect()
});
static ROTATE_DIR: Lazy<HashMap<char, char>> = Lazy::new(|| {
    vec![
        ('>', 'v'),
        ('v', '<'),
        ('<', '^'),
        ('^', '>'),
    ]
        .into_iter()
        .collect()
});
type Map = Vec<Vec<char>>;

fn read_map(filepath: &str) -> Map {
    fs::read_to_string(filepath)
        .expect("Unable to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn determine_start_position(map: &Map) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ['>', 'v', '<', '^'].contains(&ch) {
                return (i, j);
            }
        }
    }
    panic!("Unable to find start position");
}

fn is_valid_position(map: &Map, nx: isize, ny: isize) -> bool {
    if nx < 0
        || ny < 0
        || nx as usize >= map.len()
        || ny as usize >= map[0].len()
        || map[nx as usize][ny as usize] == '#'
        || map[nx as usize][ny as usize] == 'O'
    {
        return false;
    }
    true
}

fn trace_map(map: &Map, mut x: usize, mut y: usize) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut unique_paths = 0;
    let mut current_dir = map[x][y];

    loop {
        while let Some(&(dx, dy)) = MOVE_DIR.get(&current_dir) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !is_valid_position(&map, nx, ny)
            {
                break;
            }

            x = nx as usize;
            y = ny as usize;

            if !visited[x][y] {
                visited[x][y] = true;
                unique_paths += 1;
            }
        }

        if let Some(&(dx, dy)) = MOVE_DIR.get(&current_dir) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
                if !visited[x][y] {
                    visited[x][y] = true;
                    unique_paths += 1;
                }
                break;
            }
        }

        if let Some(&next_dir) = ROTATE_DIR.get(&current_dir) {
            current_dir = next_dir;
        } else {
            break;
        }
    }
    unique_paths
}

fn trace_map_and_get_positions(map: &Map, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut positions = Vec::new();
    let mut current_dir = map[x][y];

    loop {
        while let Some(&(dx, dy)) = MOVE_DIR.get(&current_dir) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !is_valid_position(&map, nx, ny)
            {
                break;
            }

            x = nx as usize;
            y = ny as usize;

            if !visited[x][y] {
                visited[x][y] = true;
                positions.push((x, y));
            }
        }

        if let Some(&next_dir) = ROTATE_DIR.get(&current_dir) {
            current_dir = next_dir;
        } else {
            break;
        }
        if x == 0 || y == 0 || x == map.len() - 1 || y == map[0].len() - 1 {
            break;
        }
    }
    positions
}

fn trace_map_and_detect_cycles(map: &Map, mut x: usize, mut y: usize) -> bool {
    let mut visited = vec![vec![(false, HashSet::new()); map[0].len()]; map.len()];
    let mut current_dir = map[x][y];

    loop {
        while let Some(&(dx, dy)) = MOVE_DIR.get(&current_dir) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if !is_valid_position(map, nx, ny) || map[nx as usize][ny as usize] == 'O' {
                break;
            }

            x = nx as usize;
            y = ny as usize;

            if visited[x][y].0 && visited[x][y].1.contains(&current_dir) {
                return true;
            } else {
                visited[x][y].0 = true;
                visited[x][y].1.insert(current_dir);
            }
        }

        if let Some(&(dx, dy)) = MOVE_DIR.get(&current_dir) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 || nx as usize >= map.len() || ny as usize >= map[0].len() {
                return false;
            }
        }

        if let Some(&next_dir) = ROTATE_DIR.get(&current_dir) {
            current_dir = next_dir;
        } else {
            break;
        }
    }

    false
}

fn process_obstacle(map: &Map, x: usize, y: usize, obstacle: (usize, usize)) -> bool {
    let mut mod_map = map.clone();
    mod_map[obstacle.0][obstacle.1] = 'O';
    trace_map_and_detect_cycles(&mod_map, x, y)
}

pub fn run() {
    let map: Map = read_map("data/aoc6.txt");
    let (x, y) = determine_start_position(&map);

    println!("Solution for Problem 6 : ");

    let path_covered = trace_map(&map, x, y);
    println!("Part 1: {}", path_covered);

    let positions = trace_map_and_get_positions(&map, x, y);
    let potential_obstacles = &positions;

    let start = Instant::now();
    let non_parallel_result: usize = potential_obstacles
        .iter()
        .filter(|&&obstacle| process_obstacle(&map, x, y, obstacle))
        .count();
    println!(
        "Part 2 Non-parallel: {}, time : {:?}",
        non_parallel_result,
        start.elapsed()
    );

    let start = Instant::now();
    let parallel_result: usize = potential_obstacles
        .par_iter()
        .filter(|&&obstacle| process_obstacle(&map, x, y, obstacle))
        .count();
    println!(
        "Part 2 Parallel: {}, time : {:?}",
        parallel_result,
        start.elapsed()
    );
    println!("***************");
}
