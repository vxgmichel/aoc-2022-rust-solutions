use std::io::{self, BufRead};

type Position = (usize, usize);

const MAXPOS: usize = 250;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Orientation {
    Right = 0,
    Down,
    Left,
    Up,
}

impl Orientation {
    fn left(&self) -> Orientation {
        match self {
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Up => Orientation::Left,
        }
    }

    fn right(&self) -> Orientation {
        match self {
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Up => Orientation::Right,
        }
    }

    fn step(&self, position: Position) -> Position {
        let (x, y) = position;
        match self {
            Orientation::Right => (x, if y == MAXPOS { 0 } else { y + 1 }),
            Orientation::Down => (if x == MAXPOS { 0 } else { x + 1 }, y),
            Orientation::Left => (x, if y == 0 { MAXPOS } else { y - 1 }),
            Orientation::Up => (if x == 0 { MAXPOS } else { x - 1 }, y),
        }
    }

    fn step_cube(&self, position: Position) -> (Position, Orientation) {
        let (x, y) = position;
        match self {
            Orientation::Right => {
                // 4-HF 6-FH
                if let Some(d) = SQUARE4.in_right(position) {
                    SQUARE6.coming_from_right(SIDE - 1 - d)
                }
                // 2-DF 4-DF
                else if let Some(d) = SQUARE2.in_right(position) {
                    SQUARE4.coming_from_bottom(d)
                }
                // 6-FH 4-HF
                else if let Some(d) = SQUARE6.in_right(position) {
                    SQUARE4.coming_from_right(SIDE - 1 - d)
                }
                // 5-GH 6-GH
                else if let Some(d) = SQUARE5.in_right(position) {
                    SQUARE6.coming_from_bottom(d)
                }
                // Regular move
                else {
                    ((x, y + 1), *self)
                }
            }
            Orientation::Down => {
                // 5-BH 4-BH
                if let Some(d) = SQUARE5.in_bottom(position) {
                    SQUARE4.coming_from_top(d)
                }
                // 6-GH 5-GH
                else if let Some(d) = SQUARE6.in_bottom(position) {
                    SQUARE5.coming_from_right(d)
                }
                // 4-DF 2-DF
                else if let Some(d) = SQUARE4.in_bottom(position) {
                    SQUARE2.coming_from_right(d)
                }
                // Regular move
                else {
                    ((x + 1, y), *self)
                }
            }
            Orientation::Left => {
                // 1-AC 3-CA
                if let Some(d) = SQUARE1.in_left(position) {
                    SQUARE3.coming_from_left(SIDE - 1 - d)
                }
                // 2-CE 3-CD
                else if let Some(d) = SQUARE2.in_left(position) {
                    SQUARE3.coming_from_top(d)
                }
                // 3-CA 1-AC
                else if let Some(d) = SQUARE3.in_left(position) {
                    SQUARE1.coming_from_left(SIDE - 1 - d)
                }
                // 5-AB 1-AB
                else if let Some(d) = SQUARE5.in_left(position) {
                    SQUARE1.coming_from_top(d)
                }
                // Regular move
                else {
                    ((x, y - 1), *self)
                }
            }
            Orientation::Up => {
                // 3-CE 2-CE
                if let Some(d) = SQUARE3.in_top(position) {
                    SQUARE2.coming_from_left(d)
                }
                // 1-AB 5-AB
                else if let Some(d) = SQUARE1.in_top(position) {
                    SQUARE5.coming_from_left(d)
                }
                // 4-BH 5-BH
                else if let Some(d) = SQUARE4.in_top(position) {
                    SQUARE5.coming_from_bottom(d)
                }
                // Regular move
                else {
                    ((x - 1, y), *self)
                }
            }
        }
    }
}

//     A---B---H
//     | 1 | 4 |
//     C---D---F
//     | 2 |
// C---E---F
// | 3 | 6 |
// A---G---H
// | 5 |
// B---H

struct Square(usize, usize);
const SIDE: usize = 50;

impl Square {
    fn coming_from_left(&self, d: usize) -> (Position, Orientation) {
        let position = (self.0 * SIDE + d, self.1 * SIDE);
        (position, Orientation::Right)
    }

    fn coming_from_right(&self, d: usize) -> (Position, Orientation) {
        let position = (self.0 * SIDE + d, self.1 * SIDE + SIDE - 1);
        (position, Orientation::Left)
    }

    fn coming_from_top(&self, d: usize) -> (Position, Orientation) {
        let position = (self.0 * SIDE, self.1 * SIDE + d);
        (position, Orientation::Down)
    }

    fn coming_from_bottom(&self, d: usize) -> (Position, Orientation) {
        let position = (self.0 * SIDE + SIDE - 1, self.1 * SIDE + d);
        (position, Orientation::Up)
    }

    fn in_top(&self, position: Position) -> Option<usize> {
        let (x, y) = position;
        if x == SIDE * self.0 && (SIDE * self.1..SIDE * self.1 + SIDE).contains(&y) {
            Some(y - SIDE * self.1)
        } else {
            None
        }
    }

    fn in_bottom(&self, position: Position) -> Option<usize> {
        let (x, y) = position;
        if x == SIDE * self.0 + SIDE - 1 && (SIDE * self.1..SIDE * self.1 + SIDE).contains(&y) {
            Some(y - SIDE * self.1)
        } else {
            None
        }
    }

    fn in_left(&self, position: Position) -> Option<usize> {
        let (x, y) = position;
        if y == SIDE * self.1 && (SIDE * self.0..SIDE * self.0 + SIDE).contains(&x) {
            Some(x - SIDE * self.0)
        } else {
            None
        }
    }

    fn in_right(&self, position: Position) -> Option<usize> {
        let (x, y) = position;
        if y == SIDE * self.1 + SIDE - 1 && (SIDE * self.0..SIDE * self.0 + SIDE).contains(&x) {
            Some(x - SIDE * self.0)
        } else {
            None
        }
    }
}

const SQUARE1: Square = Square(0, 1);
const SQUARE4: Square = Square(0, 2);
const SQUARE2: Square = Square(1, 1);
const SQUARE3: Square = Square(2, 0);
const SQUARE6: Square = Square(2, 1);
const SQUARE5: Square = Square(3, 0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Move {
    Left,
    Right,
    Forward(usize),
}

fn parse_moves(line: &str) -> Vec<Move> {
    let mut moves = vec![];
    let mut current = String::new();
    for char in line.chars() {
        match char {
            'L' | 'R' => {
                if !current.is_empty() {
                    moves.push(Move::Forward(current.parse().unwrap()));
                    current.clear();
                }
                moves.push(match char {
                    'L' => Move::Left,
                    'R' => Move::Right,
                    _ => panic!(),
                })
            }
            _ => current.push(char),
        }
    }
    if !current.is_empty() {
        moves.push(Move::Forward(current.parse().unwrap()));
        current.clear();
    }
    moves
}

fn forward(
    grid: &[Vec<char>],
    position: Position,
    orientation: Orientation,
    distance: usize,
) -> Position {
    let mut position = position;
    for _ in 0..distance {
        let mut new_position = orientation.step(position);
        loop {
            let c = grid
                .get(new_position.0)
                .and_then(|row| row.get(new_position.1))
                .cloned()
                .unwrap_or(' ');
            new_position = match c {
                ' ' => orientation.step(new_position),
                '#' => return position,
                '.' => break,
                _ => panic!(),
            }
        }
        position = new_position
    }
    position
}

fn solve1(grid: &[Vec<char>], moves: &[Move]) -> usize {
    let mut position = (0, grid[0].iter().position(|&x| x == '.').unwrap());
    let mut orientation = Orientation::Right;
    for m in moves {
        match m {
            Move::Left => orientation = orientation.left(),
            Move::Right => orientation = orientation.right(),
            Move::Forward(x) => position = forward(grid, position, orientation, *x),
        }
    }
    1000 * (position.0 + 1) + 4 * (position.1 + 1) + orientation as usize
}

fn forward2(
    grid: &[Vec<char>],
    position: Position,
    orientation: Orientation,
    distance: usize,
) -> (Position, Orientation) {
    let mut position = position;
    let mut orientation = orientation;
    for _ in 0..distance {
        let (new_position, new_orientation) = orientation.step_cube(position);
        let c = grid
            .get(new_position.0)
            .and_then(|row| row.get(new_position.1))
            .cloned()
            .unwrap_or(' ');
        match c {
            '#' => return (position, orientation),
            '.' => {
                position = new_position;
                orientation = new_orientation;
            }
            _ => panic!(),
        }
    }
    (position, orientation)
}

fn solve2(grid: &[Vec<char>], moves: &[Move]) -> usize {
    let mut position = (0, grid[0].iter().position(|&x| x == '.').unwrap());
    let mut orientation = Orientation::Right;
    for m in moves {
        match m {
            Move::Left => orientation = orientation.left(),
            Move::Right => orientation = orientation.right(),
            Move::Forward(x) => (position, orientation) = forward2(grid, position, orientation, *x),
        }
    }
    1000 * (position.0 + 1) + 4 * (position.1 + 1) + orientation as usize
}

fn main() {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .map(|x| x.chars().collect())
        .collect();
    let moves = parse_moves(&io::stdin().lock().lines().next().unwrap().unwrap());
    println!("Part 1: {}", solve1(&grid, &moves));
    println!("Part 2: {}", solve2(&grid, &moves));
}
