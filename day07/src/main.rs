use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Entry {
    File(u32),
    Directory(HashMap<String, Entry>),
}

fn build(lines: &[String]) -> Entry {
    let mut iter = lines.iter();
    let mut command = iter.next().unwrap();
    let mut root = Entry::Directory(HashMap::new());
    let mut current_path: Vec<String> = vec![];
    loop {
        match command.as_str() {
            "$ ls" => {
                let mut current_dir = &mut root;
                for part in &current_path {
                    current_dir = match current_dir {
                        Entry::Directory(x) => x
                            .entry(part.clone())
                            .or_insert_with(|| Entry::Directory(HashMap::new())),
                        Entry::File(_) => panic!(),
                    };
                }
                loop {
                    let next_line = match iter.next() {
                        Some(x) => x,
                        None => return root,
                    };
                    if next_line.starts_with('$') {
                        command = next_line;
                        break;
                    }
                    let (size, name) = next_line.split_once(' ').unwrap();
                    if size == "dir" {
                        continue;
                    }
                    match current_dir {
                        Entry::Directory(x) => {
                            x.insert(name.to_string(), Entry::File(size.parse().unwrap()))
                        }
                        Entry::File(_) => panic!(),
                    };
                }
            }
            _ if command.starts_with("$ cd ") => {
                let arg = command.strip_prefix("$ cd ").unwrap();
                match arg {
                    "/" => current_path.clear(),
                    ".." => {
                        current_path.pop().unwrap();
                    }
                    x => current_path.push(x.to_string()),
                }
                command = match iter.next() {
                    Some(x) => x,
                    None => return root,
                }
            }
            _ => panic!(),
        }
    }
}

fn get_size(entry: &Entry) -> u32 {
    match entry {
        Entry::Directory(x) => x.iter().map(|(_, v)| get_size(v)).sum(),
        Entry::File(x) => *x,
    }
}

fn rec1(entry: &Entry, threshold: u32) -> u32 {
    match entry {
        Entry::Directory(x) => {
            let size = get_size(entry);
            let subresult = x.iter().map(|(_, v)| rec1(v, threshold)).sum::<u32>();
            subresult + if size <= threshold { size } else { 0 }
        }
        Entry::File(_) => 0,
    }
}

fn rec2(entry: &Entry, threshold: u32) -> Option<u32> {
    match entry {
        Entry::Directory(x) => {
            let size = get_size(entry);
            if size < threshold {
                return None;
            }
            let subresult = x.iter().filter_map(|(_, v)| rec2(v, threshold)).min();
            Some(subresult.unwrap_or(size))
        }
        Entry::File(_) => None,
    }
}

fn solve1(lines: &[String]) -> u32 {
    let root = build(lines);
    rec1(&root, 100_000)
}

fn solve2(lines: &[String]) -> u32 {
    let root = build(lines);
    rec2(&root, get_size(&root) - 40_000_000).unwrap()
}

fn main() {
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}
