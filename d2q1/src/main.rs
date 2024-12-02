use std::fs;

fn main() {
    let content = fs::read_to_string("./input.txt")
        .expect("Should have read the file");

    let mut safes = 0;

    for report in content.lines() {
        let mut arr: Vec<i32> = report.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if check_report(&mut arr, 0) {
            safes += 1;
        }
    }

    println!("{safes}")
}

fn check_report(arr: &Vec<i32>, level: i32) -> bool {
    let mut slope = 0;
    for elem in arr.iter() {
        slope += elem - arr[0];
    }
    let mut inc = true;
    if slope < 0 {
        inc = false;
    }


    for (i, window) in arr.windows(2).enumerate() {
        let diff = window[1] - window[0];
        if diff.abs() > 3 || diff.abs() < 1 || (inc && diff < 0) || (!inc && diff > 0) {
            if level == 0 {
                let mut new_arr = arr.clone();
                new_arr.remove(i);
                let mut new_arr2 = arr.clone();
                new_arr2.remove(i + 1);
                let temp_bool = check_report(&new_arr, level+1) || check_report(&new_arr2, level+1);
                return temp_bool;
            }
            return false;
        }
    }

    true
}
