use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Operation {
    Noop,
    Addx(i32),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, ()> {
        match value {
            "noop" => Ok(Operation::Noop),
            _ if value.starts_with("addx") => Ok(Operation::Addx(
                value.split_once(' ').unwrap().1.parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

enum State {
    Fetch,
    Execute(Operation),
}

fn solve1(xs: &[Operation]) -> i32 {
    let mut it = xs.iter();
    let mut register = 1;
    let mut state = State::Fetch;
    let mut result = 0;
    for cycle in 1.. {
        if cycle % 40 == 20 {
            result += cycle * register
        }
        match state {
            State::Fetch => match it.next() {
                Some(Operation::Noop) => {}
                Some(&op @ Operation::Addx(_)) => state = State::Execute(op),
                None => break,
            },
            State::Execute(Operation::Addx(x)) => {
                register += x;
                state = State::Fetch
            }
            _ => panic!(),
        }
    }
    result
}

fn solve2(xs: &[Operation]) -> String {
    let mut it = xs.iter();
    let mut register = 1;
    let mut state = State::Fetch;
    let mut result = String::new();
    for cycle in 0.. {
        if cycle % 40 == 0 {
            result.push('\n')
        }
        if ((cycle % 40) as i32 - register).abs() <= 1 {
            result.push('\u{2588}')
        } else {
            result.push(' ')
        }
        match state {
            State::Fetch => match it.next() {
                Some(Operation::Noop) => {}
                Some(&op @ Operation::Addx(_)) => state = State::Execute(op),
                None => break,
            },
            State::Execute(Operation::Addx(x)) => {
                register += x;
                state = State::Fetch
            }
            _ => panic!(),
        }
    }
    result
}

fn main() {
    let vec: Vec<Operation> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Part 1: {}", solve1(&vec));
    println!("Part 2: {}", solve2(&vec));
}
