use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve(stream: &str, size: usize) -> u32 {
    let stream: Vec<char> = stream.chars().collect();
    for (i, x) in stream.windows(size).enumerate() {
        let s: HashSet<char> = x.iter().cloned().collect();
        if s.len() == size {
            return i as u32 + size as u32;
        }
    }
    panic!()
}

fn main() {
    let stream: String = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .next()
        .unwrap();

    println!("Part 1: {}", solve(&stream, 4));
    println!("Part 2: {}", solve(&stream, 14));
}
