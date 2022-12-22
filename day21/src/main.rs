use std::collections::HashMap;
use std::io::{self, BufRead};

type Name = String;

#[derive(Clone, Copy)]
enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone)]
enum Expression {
    Variable,
    Value(i64),
    Operation(Name, OperationType, Name),
}

enum Evaluated {
    Variable,
    Value(i64),
    Operation(Box<Evaluated>, OperationType, Box<Evaluated>),
}

impl Expression {
    fn eval(&self, map: &HashMap<Name, Expression>) -> Evaluated {
        match self {
            Expression::Variable => Evaluated::Variable,
            Expression::Value(x) => Evaluated::Value(*x),
            Expression::Operation(x, op, y) => match (map[x].eval(map), map[y].eval(map)) {
                (Evaluated::Value(x), Evaluated::Value(y)) => match op {
                    OperationType::Addition => Evaluated::Value(x + y),
                    OperationType::Subtraction => Evaluated::Value(x - y),
                    OperationType::Multiplication => Evaluated::Value(x * y),
                    OperationType::Division => Evaluated::Value(x / y),
                },
                (x, y) => Evaluated::Operation(Box::new(x), *op, Box::new(y)),
            },
        }
    }
}

fn parse_line(line: &str) -> (Name, Expression) {
    let (name, operation) = line.split_once(": ").unwrap();
    let operation = match operation.parse() {
        Ok(x) => Expression::Value(x),
        Err(_) => {
            let x = operation[0..4].to_string();
            let y = operation[7..11].to_string();
            match &operation[5..6] {
                "+" => Expression::Operation(x, OperationType::Addition, y),
                "-" => Expression::Operation(x, OperationType::Subtraction, y),
                "*" => Expression::Operation(x, OperationType::Multiplication, y),
                "/" => Expression::Operation(x, OperationType::Division, y),
                _ => panic!(),
            }
        }
    };
    (name.to_string(), operation)
}

fn solve1(map: &HashMap<Name, Expression>) -> i64 {
    if let Evaluated::Value(x) = map["root"].eval(map) {
        return x;
    }
    panic!()
}

fn rec(left: &Evaluated, right: &Evaluated) -> Option<i64> {
    match (left, right) {
        (Evaluated::Value(_), _) => rec(right, left),
        (Evaluated::Variable, Evaluated::Value(x)) => Some(*x),
        (Evaluated::Operation(a, op, b), Evaluated::Value(x)) => match (a.as_ref(), op, b.as_ref())
        {
            (a, OperationType::Addition, Evaluated::Value(b)) => rec(a, &Evaluated::Value(x - b)),
            (Evaluated::Value(a), OperationType::Addition, b) => rec(b, &Evaluated::Value(x - a)),
            (a, OperationType::Subtraction, Evaluated::Value(b)) => {
                rec(a, &Evaluated::Value(x + b))
            }
            (Evaluated::Value(a), OperationType::Subtraction, b) => {
                rec(b, &Evaluated::Value(a - x))
            }
            (a, OperationType::Multiplication, Evaluated::Value(b)) => {
                if x % b == 0 {
                    rec(a, &Evaluated::Value(x / b))
                } else {
                    None
                }
            }
            (Evaluated::Value(a), OperationType::Multiplication, b) => {
                if x % a == 0 {
                    rec(b, &Evaluated::Value(x / a))
                } else {
                    None
                }
            }
            (a, OperationType::Division, Evaluated::Value(b)) => (0..*b)
                .filter_map(|r| rec(a, &Evaluated::Value(x * b + r)))
                .next(),
            (Evaluated::Value(_), OperationType::Division, _) => panic!(),
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn solve2(map: &HashMap<Name, Expression>) -> i64 {
    let mut map = map.clone();
    let (op1, op2) = match &map["root"] {
        Expression::Operation(a, _, b) => (map[a].clone(), map[b].clone()),
        _ => panic!(),
    };
    map.insert("humn".to_string(), Expression::Variable);
    let left = op1.eval(&map);
    let right = op2.eval(&map);
    rec(&left, &right).unwrap()
}

fn main() {
    let map: HashMap<Name, Expression> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    println!("Part 1: {}", solve1(&map));
    println!("Part 2: {}", solve2(&map));
}
