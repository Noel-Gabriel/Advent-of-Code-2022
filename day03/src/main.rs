use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let grid = read_input();

    let sum_parts = find_sum(&grid);
    let gear_ratios = grid
        .iter()
        .enumerate()
        .fold(0i64, |total, (i, row)| {
            row
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == '*')
                .fold(total, |current_ratio, (j, _)| {
                    let adjs = get_adj_digits(&grid, i, j);
                    if adjs.len() != 2 { return current_ratio }
                    current_ratio + adjs.iter().fold(1i64, |v, d| v * d) 
                })
        });

    println!("Sum of part numbers: {}", sum_parts);
    println!("Sum of gear ratios: {}", gear_ratios);
}

fn read_input() -> Vec<Vec<char>> {
    let mut s = String::new();
    let _ = File::open("input03.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    s
        .trim()
        .split("\n")
        .map(|splt| splt
            .chars()
            .collect())
        .collect()
}

fn find_sum(grid: &Vec<Vec<char>>) -> i64 {
    let mut parts: i64 = 0;
    for (r, row) in grid.iter().enumerate() {
        let mut i = row.len() - 1;
        while i != 0 {
            if !row[i].is_digit(10) { i -= 1; continue; }
            let mut base = 1;
            let mut num: i64 = 0;
            let mut is_part = false;
            parts += loop { 
                if !row[i].is_digit(10) { break num; }
                is_part = is_part || check_part(grid, r, i);
                num += row[i].to_digit(10).unwrap() as i64 * base;
                base *= 10;
                if i == 0 { break num; }
                i -= 1; 
            };
            if !is_part { parts -= num; }
        }
    }
    parts
}

fn check_part(grid: &Vec<Vec<char>>, r: usize, i: usize) -> bool {
    let dir = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    for (dr, di) in dir {
        let (u, v) = (r as i32 + dr, i as i32 + di);
        if 0 <= u && u < grid.len() as i32 && 0 <= v && v < grid[r].len() as i32 {
            if !grid[u as usize][v as usize].is_digit(10) && grid[u as usize][v as usize] != '.' {
                return true
            }
        }
    }
    false
}

fn get_adj_digits(grid: &Vec<Vec<char>>, r: usize, i: usize) -> HashSet<i64> {
    let mut adjs = HashSet::new();
    let dir = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    for (dr, di) in dir {
        let (u, v) = (r as i32 + dr, i as i32 + di);
        if 0 <= u && u < grid.len() as i32 && 0 <= v && v < grid[r].len() as i32 {
            if grid[u as usize][v as usize].is_digit(10) {
                adjs.insert(get_num(grid, u as usize, v as usize));
            }
        }

    }
    adjs
}

fn get_num(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    let mut e = j;
    while e < grid[i].len() - 1 && grid[i][e + 1].is_digit(10) { e += 1; }
    let mut num: i64 = 0;
    let mut base = 1;
    while grid[i][e].is_digit(10) {
        num += grid[i][e].to_digit(10).unwrap() as i64 * base;
        base *= 10;
        if e == 0 { break; }
        e -= 1;
    }
    num
}


