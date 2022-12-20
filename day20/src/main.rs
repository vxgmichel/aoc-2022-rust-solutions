use std::io::{self, BufRead};

fn solve1(xs: &[i64], step: usize) -> i64 {
    let n = xs.len();
    let mut vec: Vec<(usize, i64)> = xs.iter().cloned().enumerate().collect();
    let mut current_index = 0;
    for i in 0..n {
        while vec[current_index].0 != i {
            current_index += 1;
            current_index %= n;
        }
        let item @ (j, value) = vec.remove(current_index);
        assert_eq!(i, j);
        let new_index = (current_index as i64 + value).rem_euclid(n as i64 - 1) as usize;
        vec.insert(new_index, item);
    }
    let mut iter = vec.iter().cloned().cycle();
    iter.find(|&(_, x)| x == 0).unwrap();
    let (_, a) = iter.nth(step - 1).unwrap();
    let (_, b) = iter.nth(step - 1).unwrap();
    let (_, c) = iter.nth(step - 1).unwrap();
    a + b + c
}

fn solve2(xs: &[i64], step: usize) -> i64 {
    let n = xs.len();
    let mut vec: Vec<(usize, i64)> = xs
        .iter()
        .cloned()
        .map(|x| x * 811589153)
        .enumerate()
        .collect();
    let mut current_index = 0;
    for _ in 0..10 {
        for i in 0..n {
            while vec[current_index].0 != i {
                current_index += 1;
                current_index %= n;
            }
            let item @ (_, value) = vec.remove(current_index);
            let new_index = (current_index as i64 + value).rem_euclid(n as i64 - 1) as usize;
            vec.insert(new_index, item);
        }
    }
    let mut iter = vec.iter().cloned().cycle();
    iter.find(|&(_, x)| x == 0).unwrap();
    let (_, a) = iter.nth(step - 1).unwrap();
    let (_, b) = iter.nth(step - 1).unwrap();
    let (_, c) = iter.nth(step - 1).unwrap();
    a + b + c
}

fn main() {
    let vec: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    println!("Part 1: {}", solve1(&vec, 1000));
    println!("Part 1: {}", solve2(&vec, 1000));
}
