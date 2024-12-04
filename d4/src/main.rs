use std::{collections::HashSet, fs};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct XmasOccurence {
    row: usize,
    col: usize,
    dx: i32,
    dy: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct MasOccurence {
    row: usize,
    col: usize,
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    part1(&content);
    part2(&content);
}

fn part2(content: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let data = find_mas(&grid);
    println!("{:#?}", data.len());
}

fn find_mas(grid: &Vec<Vec<char>>) -> Vec<MasOccurence> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut occurences = HashSet::new();

    for row in 0..rows {
        for col in 0..cols {
            if let Some(occurence) = check_mas(grid, row, col) {
                occurences.insert(occurence);
            }
        }
    }

    occurences.into_iter().collect()
}

fn check_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Option<MasOccurence> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let dirs = [(1, 1), (1, -1)];

    for &(dy, dx) in &dirs {
        let end_row = row as i32 + dy;
        let end_col = col as i32 + dx;

        let start_row = row as i32 - dy;
        let start_col = col as i32 - dx;

        if end_row >= rows
            || end_row < 0
            || end_col >= cols
            || end_col < 0
            || start_row >= rows
            || start_row < 0
            || start_col >= cols
            || start_col < 0
            || grid[row][col] != 'A'
        {
            return None;
        }

        if grid[end_row as usize][end_col as usize] != 'M'
            && grid[end_row as usize][end_col as usize] != 'S'
        {
            return None;
        }

        if grid[end_row as usize][end_col as usize] == 'M'
            && grid[start_row as usize][start_col as usize] != 'S'
        {
            return None;
        }

        if grid[end_row as usize][end_col as usize] == 'S'
            && grid[start_row as usize][start_col as usize] != 'M'
        {
            return None;
        }
    }

    Some(MasOccurence { row, col })
}

fn part1(content: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let data = find_occurence_xmas(&grid);
    println!("{}", data.len());
}

fn find_occurence_xmas(grid: &Vec<Vec<char>>) -> Vec<XmasOccurence> {
    let rows = grid.len();
    let cols = grid[0].len();
    let target = "XMAS";
    let mut occurences = HashSet::new();

    let dirs = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dy, dx) in &dirs {
                if let Some(occurence) = check_xmas(grid, row, col, dy, dx, target) {
                    occurences.insert(occurence);
                }
            }
        }
    }

    occurences.into_iter().collect()
}

fn check_xmas(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    dy: i32,
    dx: i32,
    target: &str,
) -> Option<XmasOccurence> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let end_row = row as i32 + dy * (target.len() as i32 - 1);
    let end_col = col as i32 + dx * (target.len() as i32 - 1);

    if end_row >= rows || end_row < 0 || end_col >= cols || end_col < 0 {
        return None;
    }

    for (i, c) in target.chars().enumerate() {
        let curr_row = (row as i32 + dy * i as i32) as usize;
        let curr_col = (col as i32 + dx * i as i32) as usize;

        if grid[curr_row][curr_col] != c {
            return None;
        }
    }

    Some(XmasOccurence { row, col, dx, dy })
}
