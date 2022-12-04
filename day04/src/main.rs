use std::io::{self, BufRead};

fn solve1(xs: &[(u32, u32, u32, u32)]) -> u32 {
    let mut result = 0;
    for (a, b, c, d) in xs {
        if a <= c && d <= b || c <= a && b <= d {
            result += 1;
        }
    }
    result
}

fn solve2(xs: &[(u32, u32, u32, u32)]) -> u32 {
    let mut result = 0;
    for (a, b, c, d) in xs {
        if !(b < c || d < a) {
            result += 1;
        }
    }
    result
}

fn main() {
    let vec: Vec<(u32, u32, u32, u32)> = io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let (first, second) = line.split_once(",").unwrap();
            let (a, b) = first.split_once("-").unwrap();
            let (c, d) = second.split_once("-").unwrap();
            let a = a.parse::<u32>().unwrap();
            let b = b.parse::<u32>().unwrap();
            let c = c.parse::<u32>().unwrap();
            let d = d.parse::<u32>().unwrap();
            (a, b, c, d)
        })
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
