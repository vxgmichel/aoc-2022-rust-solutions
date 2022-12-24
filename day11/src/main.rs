fn inital_state() -> [Vec<u64>; 8] {
    [
        vec![63, 57],
        vec![82, 66, 87, 78, 77, 92, 83],
        vec![97, 53, 53, 85, 58, 54],
        vec![50],
        vec![64, 69, 52, 65, 73],
        vec![57, 91, 65],
        vec![67, 91, 84, 78, 60, 69, 99, 83],
        vec![58, 78, 69, 65],
    ]
}

fn update_item(i: usize, item: u64) -> u64 {
    match i {
        0 => item * 11,
        1 => item + 1,
        2 => item * 7,
        3 => item + 3,
        4 => item + 6,
        5 => item + 5,
        6 => item * item,
        7 => item + 7,
        _ => panic!(),
    }
}

fn new_index(i: usize, item: u64) -> usize {
    const TEST_INFO: [(u64, u64, u64); 8] = [
        (7, 6, 2),
        (11, 5, 0),
        (13, 4, 3),
        (3, 1, 7),
        (17, 3, 7),
        (2, 0, 6),
        (5, 2, 4),
        (19, 5, 1),
    ];
    let (a, b, c) = TEST_INFO[i];
    (if item % a == 0 { b } else { c }) as usize
}

fn solve(divisor: u64, rounds: usize) -> u64 {
    let modulo = 7 * 11 * 13 * 3 * 17 * 2 * 5 * 19;
    let mut monkeys = inital_state();
    let mut scores: [u64; 8] = Default::default();
    for _round in 0..rounds {
        for i in 0..8 {
            for mut item in monkeys[i].clone() {
                item = update_item(i, item);
                if divisor == 1 {
                    item %= modulo;
                } else {
                    item /= divisor;
                }
                let j = new_index(i, item);
                monkeys[j].push(item);
                scores[i] += 1;
            }
            monkeys[i].clear()
        }
    }
    scores.sort();
    let mut it = scores.iter().rev();
    it.next().unwrap() * it.next().unwrap()
}

fn main() {
    println!("Part 1: {}", solve(3, 20));
    println!("Part 2: {}", solve(1, 10000));
}
