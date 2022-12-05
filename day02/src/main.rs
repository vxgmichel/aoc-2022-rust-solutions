use std::io::{self, BufRead};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl TryFrom<char> for Hand {
    type Error = ();

    fn try_from(v: char) -> Result<Self, Self::Error> {
        match v {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err(()),
        }
    }
}

fn part1(xs: &[(Hand, Hand)]) -> u32 {
    let mut score = 0;
    for &(x, y) in xs {
        score += y as u32 + 1;
        score += (y as u32 + (3 - x as u32) + 1) % 3 * 3;
    }
    score
}

fn part2(xs: &[(Hand, Hand)]) -> u32 {
    let mut score = 0;
    for &(x, y) in xs {
        score += (y as u32) * 3;
        score += 1 + (x as u32 + y as u32 + 2) % 3;
    }
    score
}

fn main() {
    let vec: Vec<(Hand, Hand)> = io::stdin()
        .lock()
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| {
            let (a, b) = x.split_once(' ')?;
            let x = a.chars().next()?.try_into().ok()?;
            let y = b.chars().next()?.try_into().ok()?;
            Some((x, y))
        })
        .collect();
    println!("Part 1: {}", part1(&vec));
    println!("Part 2: {}", part2(&vec));
}
