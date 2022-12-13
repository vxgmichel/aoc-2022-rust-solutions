use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Token {
    Enter,
    Exit,
    Next,
    Value(u32),
}

type Packet = Vec<Token>;
type PacketPair = (Packet, Packet);

fn parse_line(line: &str) -> Packet {
    let mut temp = String::new();
    let mut result = vec![];
    for c in line.chars() {
        match c {
            '[' | ',' | ']' => {
                if !temp.is_empty() {
                    result.push(Token::Value(temp.parse().unwrap()));
                    temp.clear();
                }
                match c {
                    '[' => result.push(Token::Enter),
                    ']' => result.push(Token::Exit),
                    ',' => result.push(Token::Next),
                    _ => panic!(),
                }
            }
            '0'..='9' => temp.push(c),
            _ => panic!(),
        }
    }
    assert!(temp.is_empty());
    result
}

fn compare_until_exit(
    left: &mut dyn Iterator<Item = &Token>,
    right: &mut dyn Iterator<Item = &Token>,
) -> Ordering {
    match compare_item(left, right) {
        Ok(Ordering::Less) => Ordering::Less,
        Ok(Ordering::Equal) => compare_until_exit(left, right),
        Ok(Ordering::Greater) => Ordering::Greater,
        Err((Token::Exit, Token::Exit)) => Ordering::Equal,
        Err((Token::Exit, _)) => Ordering::Less,
        Err((_, Token::Exit)) => Ordering::Greater,
        Err((Token::Next, Token::Next)) => compare_until_exit(left, right),
        x => panic!("{:?}", x),
    }
}

fn compare_item(
    left: &mut dyn Iterator<Item = &Token>,
    right: &mut dyn Iterator<Item = &Token>,
) -> Result<Ordering, (Token, Token)> {
    Ok(match (left.next().unwrap(), right.next().unwrap()) {
        (Token::Value(x), Token::Value(y)) => x.cmp(y),
        (Token::Enter, Token::Enter) => compare_until_exit(left, right),
        (x @ Token::Value(_), Token::Enter) => {
            let vec = &vec![*x, Token::Exit];
            let left = &mut vec.iter();
            compare_until_exit(left, right)
        }
        (Token::Enter, x @ Token::Value(_)) => {
            let vec = &vec![*x, Token::Exit];
            let right = &mut vec.iter();
            compare_until_exit(left, right)
        }
        (&x, &y) => return Err((x, y)),
    })
}

fn compare(left: &[Token], right: &[Token]) -> Ordering {
    let mut left_it = left.iter();
    let mut right_it = right.iter();
    compare_item(&mut left_it, &mut right_it).unwrap()
}

fn solve1(pairs: &[PacketPair]) -> u32 {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| compare(left, right).is_le())
        .map(|(i, _)| i as u32 + 1)
        .sum()
}

fn solve2(mut vec: Vec<Packet>) -> u32 {
    let divider1 = parse_line("[[2]]");
    let divider2 = parse_line("[[6]]");
    vec.push(divider1.clone());
    vec.push(divider2.clone());
    vec.sort_by(|x, y| compare(x, y));
    let index1 = vec.iter().position(|x| x == &divider1).unwrap() + 1;
    let index2 = vec.iter().position(|x| x == &divider2).unwrap() + 1;
    index1 as u32 * index2 as u32
}

fn main() {
    let vec: Vec<Packet> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.is_empty())
        .map(|x| parse_line(&x))
        .collect();
    let pairs: Vec<PacketPair> = vec
        .chunks(2)
        .map(|x| (x[0].clone(), x[1].clone()))
        .collect();
    println!("Part 1: {}", solve1(&pairs));
    println!("Part 2: {}", solve2(vec));
}
