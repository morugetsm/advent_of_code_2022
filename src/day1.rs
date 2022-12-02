pub fn part1() -> u32 {
    let input = std::fs::read_to_string("./input/day1.txt").unwrap();

    let mut elves = Vec::new();
    elves.push(Vec::new());

    for cal in input.lines() {
        if cal.is_empty() {
            elves.push(Vec::new());
        } else {
            elves.last_mut().unwrap().push(cal.parse::<u32>().unwrap());
        }
    }

    elves.iter().fold(0, |acc, curr| acc.max(curr.iter().sum()))
}

pub fn part2() -> u32 {
    let input = std::fs::read_to_string("./input/day1.txt").unwrap();

    let mut elves = Vec::new();
    elves.push(Vec::new());

    for cal in input.lines() {
        if cal.is_empty() {
            elves.push(Vec::new());
        } else {
            elves.last_mut().unwrap().push(cal.parse::<u32>().unwrap());
        }
    }

    let mut calrories: std::collections::BinaryHeap<u32> =
        elves.iter().map(|elf| elf.iter().sum()).collect();

    calrories.pop().unwrap() + calrories.pop().unwrap() + calrories.pop().unwrap()
}
