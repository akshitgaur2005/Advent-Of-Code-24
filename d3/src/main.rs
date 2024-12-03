use std::fs;

use regex::Regex;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let content2 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //part1(&content);
    part2(&content);
}

fn part1(content: &str) {
    let re = Regex::new(r"mul\((?<dig1>\d{1,3}),(?<dig2>\d{1,3})\)").unwrap();

    let captured: Vec<(i32, i32)> = re.captures_iter(content).map(|temp| {
        let digit1 = temp.name("dig1").unwrap().as_str().parse().unwrap();
        let digit2 = temp.name("dig2").unwrap().as_str().parse().unwrap();
        (digit1, digit2)
    }).collect();

    //println!("{captured:?}");

    let mut total_sum = 0;
    for (a, b) in captured.iter() {
        total_sum += a * b;
    }
    println!("{total_sum}");
}

fn part2(content: &str) {
    let re = Regex::new(r"(?<data>(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\)))").unwrap();

    let instructs: Vec<&str> = re.captures_iter(content).map(|temp| {
        temp.name("data").unwrap().as_str()
    }).collect();

    let mut total_sum = 0;

    let mut to_do = true;
    for elem in instructs.iter() {
        if *elem == "do()" {
            to_do = true;
            continue;
        } else if *elem == "don't()" {
            to_do = false;
            continue;
        }

        if to_do {
            let re2 = Regex::new(r"mul\((?<dig1>\d{1,3}),(?<dig2>\d{1,3})\)").unwrap();
            let cap = re2.captures(elem).unwrap();
            let a: i32 = cap.name("dig1").unwrap().as_str().parse().unwrap();
            let b: i32 = cap.name("dig2").unwrap().as_str().parse().unwrap();
            total_sum += a * b;
        }
    }
    println!("{total_sum}");
}
