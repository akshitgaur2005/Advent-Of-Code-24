use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let _content = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    // part1(&content);
    part2(&content);
}

fn _part1(content: &str) {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    let mut pos = (0, 0);

    for (i, line) in content.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '.' => 0, // Empty Space
                '#' => 1, // Obstacle
                '^' => {
                    pos = (i, j);
                    0
                }
                _ => 8,
            });
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let dirs: [(i32, i32); 4] = [
        (-1, 0), // dx, dy
        (0, 1),
        (1, 0),
        (0, -1),
    ];
    let mut dir_num = 0; // UP
    println!("{pos:?}");
    println!("{}", grid[6][1]);
    loop {
        let dir = dirs[dir_num];
        grid[pos.0][pos.1] = 3; // Visited

        let y: i32 = pos.1 as i32 + dir.1;
        let x: i32 = pos.0 as i32 + dir.0;

        if y >= cols as i32 || y < 0 || x >= rows as i32 || x < 0 {
            break;
        }

        match grid[x as usize][y as usize] {
            0 | 3 => {
                pos = (x as usize, y as usize);
            }
            1 => {
                dir_num = (dir_num + 1) % dirs.len();
            }
            _ => (),
        }
    }

    let mut xs = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 3 {
                xs += 1;
            }
        }
    }
    println!("{xs}")
}

fn part2(content: &str) {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    let mut pos = (0, 0);

    for (i, line) in content.lines().enumerate() {
        let mut row: Vec<u8> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '.' => 0, // Empty Space
                '#' => 1, // Obstacle
                '^' => {
                    pos = (i, j);
                    0
                }
                _ => 8,
            });
        }
        grid.push(row);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let dirs: [(i32, i32); 4] = [
        (-1, 0), // dx, dy
        (0, 1),
        (1, 0),
        (0, -1),
    ];
    let mut dir_num = 0; // UP
    let mut num_obstacle = 0;
    let mut rays: Vec<(usize, usize)> = Vec::new();
    loop {
        let dir = dirs[dir_num];
        grid[pos.0][pos.1] = 3 + dir_num as u8; // Visited

        let y: i32 = pos.1 as i32 + dir.1;
        let x: i32 = pos.0 as i32 + dir.0;

        if y >= cols as i32 || y < 0 || x >= rows as i32 || x < 0 {
            break;
        }

        match grid[x as usize][y as usize] {
            1 => {
                dir_num = (dir_num + 1) % dirs.len();
                continue;
            }
            _ => {
                pos = (x as usize, y as usize);
            }
        }

        let check_dir_num = (dir_num + 1) % dirs.len();
        if raycast(&grid, pos, dirs, check_dir_num) {
            rays.push(pos);
        }

        pos = (x as usize, y as usize);
    }

    rays.sort();
    rays.dedup();

    println!("{}", rays.len());
}

fn raycast(
    grid: &Vec<Vec<u8>>,
    pos: (usize, usize),
    dirs: [(i32, i32); 4],
    dir_num: usize,
) -> bool {
    let mut grid = grid.clone();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut pos = pos;
    let dir = dirs[dir_num];
    loop {
        grid[pos.0][pos.1] = 3 + dir_num as u8;
        let x: i32 = pos.0 as i32 + dir.0;
        let y: i32 = pos.1 as i32 + dir.1;

        if y as usize >= cols || y < 0 || x as usize >= rows || x < 0 {
            return false;
        }

        let found = grid[x as usize][y as usize];

        if found == 1 {
            let to_return = raycast(
                &grid,
                (x as usize, y as usize),
                dirs,
                (dir_num + 1) % dirs.len(),
            );
            return to_return;
        }

        if found as usize == dir_num + 3 {
            return true;
        } else if found as usize == ((dir_num + 1) % dirs.len()) + 3 {
            let x2: i32 = x + dir.0;
            let y2: i32 = y + dir.1;

            if y2 as usize >= cols || y2 < 0 || x2 as usize >= rows || x2 < 0 {
                return false;
            }

            if grid[x2 as usize][y2 as usize] == 1 {
                return true;
            }
        }

        pos = (x as usize, y as usize);
    }
}
