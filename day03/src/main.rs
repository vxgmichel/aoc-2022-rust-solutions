use std::collections::HashSet;
use std::io::{self, BufRead};

fn solve1(xs: &[String]) -> u32 {
    let mut result = 0;
    for x in xs {
        let n = x.len();
        let first: HashSet<char> = x.chars().take(n / 2).collect();
        let second: HashSet<char> = x.chars().skip(n / 2).collect();
        for &x in first.intersection(&second) {
            result += match x {
                'a'..='z' => x as u32 - 'a' as u32 + 1,
                'A'..='Z' => x as u32 - 'A' as u32 + 26 + 1,
                _ => panic!(),
            };
        }
    }
    result
}

fn solve2(xs: &[String]) -> u32 {
    let mut result = 0;
    for x in xs.chunks(3) {
        let first: HashSet<char> = x[0].chars().collect();
        let second: HashSet<char> = x[1].chars().collect();
        let third: HashSet<char> = x[2].chars().collect();
        for &x in first
            .intersection(&second)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&third)
        {
            result += match x {
                'a'..='z' => x as u32 - 'a' as u32 + 1,
                'A'..='Z' => x as u32 - 'A' as u32 + 26 + 1,
                _ => panic!(),
            };
        }
    }
    result
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().filter_map(|x| x.ok()).collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
