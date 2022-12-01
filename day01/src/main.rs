use std::io::{self, BufRead};

fn solve(xs: &[u32], n: usize) -> u32 {
    let mut ys: Vec<u32> = xs.iter().fold(vec![0], |mut acc, &element| {
        if element == 0 {
            acc.push(0)
        } else {
            let last = acc.len() - 1;
            acc[last] += element;
        }
        acc
    });
    ys.sort();
    ys[ys.len() - n..].iter().sum()
}

fn main() {
    let vec: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse().ok().or(Some(0)))
        .collect();
    println!("Part 1: {}", solve(&vec, 1));
    println!("Part 2: {}", solve(&vec, 3));
}
