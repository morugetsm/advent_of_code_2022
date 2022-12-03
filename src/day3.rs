pub fn part1() -> u32 {
    let input = std::fs::read_to_string("./input/day3.txt").unwrap();

    let mut result = 0;

    for line in input.lines() {
        let (first, last) = line.split_at(line.len() / 2);

        let same = first.chars().find(|c| last.contains(*c)).unwrap();

        result += match same {
            'a'..='z' => same as u32 - 'a' as u32 + 1,
            'A'..='Z' => same as u32 - 'A' as u32 + 27,
            _ => 0,
        }
    }

    result
}

pub fn part2() -> u32 {
    let input = std::fs::read_to_string("./input/day3.txt").unwrap();

    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    for group in lines.chunks_exact(3) {
        if let &[first, second, third] = group {
            let same = first
                .chars()
                .find(|c| second.contains(*c) && third.contains(*c))
                .unwrap();

            result += match same {
                'a'..='z' => same as u32 - 'a' as u32 + 1,
                'A'..='Z' => same as u32 - 'A' as u32 + 27,
                _ => 0,
            }
        }
    }

    result
}
