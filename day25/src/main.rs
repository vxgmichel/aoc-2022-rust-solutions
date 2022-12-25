use std::io::{self, BufRead};

fn snafu_to_int(string: &str) -> u64 {
    if string.is_empty() {
        return 0;
    }
    let n = string.len();
    let new_string = &string[0..n - 1];
    match &string[n - 1..n] {
        "=" => 5 * snafu_to_int(new_string) - 2,
        "-" => 5 * snafu_to_int(new_string) - 1,
        "0" => 5 * snafu_to_int(new_string),
        "1" => 5 * snafu_to_int(new_string) + 1,
        "2" => 5 * snafu_to_int(new_string) + 2,
        _ => panic!(),
    }
}

fn int_to_snafu(mut value: u64) -> String {
    let mut result = String::new();
    while value != 0 {
        let (c, carry) = match value % 5 {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => panic!(),
        };
        result.push(c);
        value /= 5;
        value += carry;
    }
    if result.is_empty() {
        result.push('0')
    }
    return result.chars().rev().collect();
}

fn solve(xs: &[String]) -> String {
    int_to_snafu(xs.iter().map(|x| snafu_to_int(x)).sum())
}

fn main() {
    let vec: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    println!("Part 1: {}", solve(&vec));
}
