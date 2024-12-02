use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt")
        .expect("Should have read the file");
    let mut arr1 = [0; 1000];
    let mut arr2 = [0; 1000];
    for (i, line) in contents.lines().enumerate() {
        let mut ints = line.split_whitespace();
        arr1[i] = ints.next().unwrap_or("").parse().expect("Should be a number");
        arr2[i] = ints.next().unwrap_or("").parse().expect("Should be a number");
    }
    arr1.sort();
    arr2.sort();
    let mut hash = HashMap::new();

    for num in arr2.iter() {
        let count = hash.entry(num).or_insert(0);
        *count += 1;
    }

    let mut sim_score = 0;

    for num in arr1.iter() {
        sim_score += num * hash.get(num).copied().unwrap_or(0);
    }

    println!("{sim_score}");
}
