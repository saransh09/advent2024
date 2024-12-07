use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn read_word_map(file_name: &str) -> Vec<Vec<char>> {
    let path = Path::new(file_name);
    let file = File::open(&path).expect("File not found");
    let lines = io::BufReader::new(file).lines();

    let word_map = lines
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    word_map
}

fn is_inbounds(x: i32, y: i32, word_map: &[Vec<char>]) -> bool {
    0 <= x && x < word_map.len() as i32 && 0 <= y && y < word_map[0].len() as i32
}

fn check_1(word_map: &[Vec<char>], x: usize, y: usize) -> i32 {
    let mut matches = 0;
    if word_map[x][y] == 'X' {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if is_inbounds(x as i32 + dx * 3, y as i32 + dy * 3, word_map) {
                    let pattern: String = (0..4).map(|k| {
                        let nx = x as i32 + k * dx;
                        let ny = y as i32 + k * dy;
                        word_map[nx as usize][ny as usize]
                    })
                        .collect();

                    if pattern == "XMAS" {
                        matches += 1;
                    }
                }
            }
        }
    }
    matches
}


fn search_xmas(word_map: &Vec<Vec<char>>) -> usize {
    let mut matches: usize = 0;
    for i in 0..word_map.len() {
        for j in 0..word_map[0].len() {
            let mat = check_1(&word_map, i, j);
            if mat > 0 {
                matches += mat as usize;
            }
        }
    }
    matches
}

fn search_x_mas(word_map: &Vec<Vec<char>>) -> usize {
    let mut matches: usize = 0;

    for x in 0..word_map.len() {
        for y in 0..word_map[0].len() {
            if word_map[x][y] != 'A' {
                continue;
            }

            if is_inbounds(x as i32 + 1, y as i32 + 1, word_map) &&
                is_inbounds(x as i32 + 1, y as i32 - 1, word_map) &&
                is_inbounds(x as i32 - 1, y as i32 - 1, word_map) &&
                is_inbounds(x as i32 - 1, y as i32 + 1, word_map) {
                let diag1 = (word_map[x - 1][y - 1], word_map[x + 1][y + 1]);
                let diag2 = (word_map[x + 1][y - 1], word_map[x - 1][y + 1]);
                if matches!(diag1, ('M', 'S') | ('S', 'M')) &&
                    matches!(diag2, ('M', 'S') | ('S', 'M')) {
                    matches += 1;
                }
            }
        }
    }
    matches
}

pub fn run() {
    println!("Solution for Problem 4 : ");
    let word_map = read_word_map("data/aoc4.txt");
    let matches = search_xmas(&word_map);
    println!("Part 1: {}", matches);
    println!("Part 2: {}", search_x_mas(&word_map));
    println!("***************");
}