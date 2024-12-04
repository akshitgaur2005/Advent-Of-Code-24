use std::{borrow::Borrow, fs};

fn main() {
    let _content = fs::read_to_string("./input.txt").unwrap();
    let content2 = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    part1(&content2);
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

    let mut count = 0;

    // Horizontal Search (Forward + Backward)
    for mut row in &grid {
        let line = &String::from_iter(row.iter());
        let mut found_idx = vec![0; line.len()];
        let data1 = check_string(line, "XMAS", &mut found_idx);
        println!("{:?}", data1.1);
        //let data2 = check_string(line, "SAMX", data1.1);
        //let line = &mask(line, data2.1);
        row = line.chars();
    }

    // Vertical Search (Forward + Backward)
    for i in 0..grid[0].len() {
        //let line = &col_to_line(&grid, i);
        //count += check_string(line, "XMAS") + check_string(line, "SAMX");
    }

    //
    let to_check: String = String::from_iter(grid[0].iter());
    let mut found_idx = vec![0; to_check.len()];
    println!("{}", check_string(&to_check, "XMAS", &mut found_idx).0);
/*
    println!(
        "{}",
        check_string(
            &String::from("XXOMASNXXMAS"),
            "XMAS",
            &mut [0; "XXOMASNXXNAS".len()]
        ).0
    );
*/
    //println!("{}", check_string(&col_to_line(&grid, 9), "XMAS"));
    println!("{}", count);
}

fn check_string<'a>(line: &str, find: &str, found_idx: &'a mut [i32]) -> (i32, &'a [i32]) {
    // MSAMXMSMSA
    // XMASAMXAMM
    let line = &mask(line, found_idx);
    println!("{line} \t- {find} \t- {found_idx:?}");

    if find.is_empty() {
        return (1, found_idx);
    }
    let mut count = 0;
    for (i, c) in line.chars().enumerate() {
        let n = match find.chars().next() {
            Some(m) => m,
            None => ' ',
        };
        if c == n && found_idx[i] == 0 {
            count += check_string(&line[i + 1..], &find[1..], &mut found_idx[i + 1..]).0;
            found_idx[i] = 1;
        }
    }
    (count, found_idx)
}

fn col_to_line(grid: &Vec<Vec<char>>, col_num: usize) -> String {
    let mut line = String::new();
    for i in 0..grid.len() {
        line.push(grid[i][col_num]);
    }
    line
}

fn mask(line: &str, mask: &[i32]) -> String {
    line.chars()
        .enumerate()
        .map(|(idx, ch)| {
            if mask[idx] == 1 {
                '.'.to_string()
            } else {
                ch.to_string()
            }
        })
        .collect()
}
