use std::fs;

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./src/q1-d1.txt")
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
    let mut total_sum = 0;
    for (a1, a2) in arr1.iter().zip(arr2.iter()) {
        let temp: i32 = a1 - a2;
        total_sum += temp.abs();
    }
    println!("{total_sum}");
}
