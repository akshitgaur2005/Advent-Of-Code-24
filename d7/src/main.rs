use std::{collections::VecDeque, fs};

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let _content = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    part1(&content);
}

fn part1(content: &str) {
    let mut answer = 0;
    for line in content.lines() {
        let mut parts = line.split(":");
        let target: u64 = parts.next().unwrap().parse().unwrap();
        let inputs: Vec<u64> = parts.next().unwrap().trim().split(" ").map(|s| s.parse().unwrap()).collect();
        if traverse_tree(target, 0, &inputs, '+') || traverse_tree(target, 0, &inputs, '*') {
            answer += target;
        }
    }
    println!("{answer}");
}

fn traverse_tree(target: u64, current: u64, inputs: &[u64], op: char) -> bool {
    if op == '+' {
        if inputs.len() == 1 {
            if current + inputs[0] == target {
                return true;
            }
            return false;
        } else {
            return traverse_tree(target, current + inputs[0], &inputs[1..], '+') || traverse_tree(target, current + inputs[0], &inputs[1..], '*') || traverse_tree(target, current + inputs[0], &inputs[1..], '|');
        }
    } else if op == '*' {
        if inputs.len() == 1 {
            if current * inputs[0] == target {
                return true;
            }
            return false;
        } else {
            return traverse_tree(target, current * inputs[0], &inputs[1..], '+') || traverse_tree(target, current * inputs[0], &inputs[1..], '*') || traverse_tree(target, current * inputs[0], &inputs[1..], '|');
        }
    } else if op == '|' {
        let new: u64 = format!("{}{}", current, inputs[0]).parse().unwrap();

        if inputs.len() == 1 {
            if new == target {
                return true;
            }
            return false;
        } else {
            return traverse_tree(target, new, &inputs[1..], '+') || traverse_tree(target, new, &inputs[1..], '*') || traverse_tree(target, new, &inputs[1..], '|')
        }
    }
    false
}
