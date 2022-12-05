use std::collections::VecDeque;

// TODO: So many inefficient codes... fix it!
pub fn part1() -> String {
    let input = std::fs::read_to_string("./input/day5.txt").unwrap();

    let input_crates: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();
    let input_instructions: Vec<&str> = input.lines().skip(input_crates.len() + 1).collect();

    let mut sangjas: Vec<VecDeque<char>> = Vec::new();

    for (idx, line) in input_crates.iter().enumerate() {
        for (column, sangja) in line.as_bytes().chunks(4).enumerate() {
            if idx == 0 {
                sangjas.push(VecDeque::new());
            }
            if sangja[1].is_ascii_alphabetic() {
                sangjas[column].push_front(sangja[1] as char);
            }
        }
    }

    for inst in input_instructions {
        let parts: Vec<usize> = inst
            .split_ascii_whitespace()
            .filter(|seg| seg.as_bytes().iter().all(|c| (*c as char).is_numeric()))
            .map(|part| part.parse().unwrap())
            .collect();

        if let [count, from, to] = parts[..] {
            for _ in 0..count {
                let c = sangjas[from - 1].pop_back().unwrap();
                sangjas[to - 1].push_back(c);
            }
        }
    }

    let result = sangjas.iter_mut().fold(String::new(), |mut acc, val| {
        if let Some(v) = val.pop_back() {
            acc.push(v);
        }
        acc
    });

    result
}

pub fn part2() -> String {
    let input = std::fs::read_to_string("./input/day5.txt").unwrap();

    let input_crates: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();
    let input_instructions: Vec<&str> = input.lines().skip(input_crates.len() + 1).collect();

    let mut sangjas: Vec<VecDeque<char>> = Vec::new();

    for (idx, line) in input_crates.iter().enumerate() {
        for (column, sangja) in line.as_bytes().chunks(4).enumerate() {
            if idx == 0 {
                sangjas.push(VecDeque::new());
            }
            if sangja[1].is_ascii_alphabetic() {
                sangjas[column].push_front(sangja[1] as char);
            }
        }
    }

    for inst in input_instructions {
        let parts: Vec<usize> = inst
            .split_ascii_whitespace()
            .filter(|seg| seg.as_bytes().iter().all(|c| (*c as char).is_numeric()))
            .map(|part| part.parse().unwrap())
            .collect();

        // changed
        if let [count, from, to] = parts[..] {
            let split_at = sangjas[from - 1].len() - count;
            let moving = sangjas[from - 1].split_off(split_at);
            sangjas[to - 1].extend(moving);
        }
    }

    let result = sangjas.iter_mut().fold(String::new(), |mut acc, val| {
        if let Some(v) = val.pop_back() {
            acc.push(v);
        }
        acc
    });

    result
}
