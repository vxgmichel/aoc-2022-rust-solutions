use std::io::{self, BufRead};

fn solve1(heaps: &[Vec<char>; 9], operations: &[(usize, usize, usize)]) -> String {
    let mut heaps = heaps.clone();
    for &(a, b, c) in operations {
        for _ in 0..a {
            let x = heaps[b].pop().unwrap();
            heaps[c].push(x);
        }
    }
    heaps.iter().map(|x| x.last().unwrap()).collect()
}

fn solve2(heaps: &[Vec<char>; 9], operations: &[(usize, usize, usize)]) -> String {
    let mut heaps = heaps.clone();
    for &(a, b, c) in operations {
        let mut temp = vec![];
        for _ in 0..a {
            let x = heaps[b].pop().unwrap();
            temp.push(x);
        }
        heaps[c].extend(temp.iter().rev())
    }
    heaps.iter().map(|x| x.last().unwrap()).collect()
}

fn main() {
    let mut heaps: [Vec<char>; 9] = Default::default();
    for line in io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| x.contains('['))
    {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                heaps[i].push(c);
            }
        }
    }
    for heap in heaps.iter_mut() {
        heap.reverse()
    }
    let operations: Vec<(usize, usize, usize)> = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .skip_while(|x| x.is_empty())
        .map(|x| {
            let mut it = x.split(' ').skip(1).step_by(2);
            let a = it.next().unwrap().parse::<usize>().unwrap();
            let b = it.next().unwrap().parse::<usize>().unwrap();
            let c = it.next().unwrap().parse::<usize>().unwrap();
            (a, b - 1, c - 1)
        })
        .collect();

    println!("Part 1: {}", solve1(&heaps, &operations));
    println!("Part 2: {}", solve2(&heaps, &operations));
}
