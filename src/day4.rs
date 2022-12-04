pub fn part1() -> u32 {
    let input = std::fs::read_to_string("./input/day4.txt").unwrap();

    let mut result = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let (first, last) = (split[0], split[1]);

        let f_range: Vec<&str> = first.split('-').collect();
        let l_range: Vec<&str> = last.split('-').collect();

        let (fs, fe, ls, le): (u32, u32, u32, u32) = (
            f_range[0].parse().unwrap(),
            f_range[1].parse().unwrap(),
            l_range[0].parse().unwrap(),
            l_range[1].parse().unwrap(),
        );

        if (fs <= ls && fe >= le) || (fs >= ls && fe <= le) {
            result += 1;
        }
    }

    result
}

// 852
pub fn part2() -> u32 {
    let input = std::fs::read_to_string("./input/day4.txt").unwrap();

    let mut result = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(',').collect();
        let (first, last): (Vec<&str>, Vec<&str>) =
            (split[0].split('-').collect(), split[1].split('-').collect());

        let f_range = std::ops::RangeInclusive::new(
            first[0].parse::<u32>().unwrap(),
            first[1].parse::<u32>().unwrap(),
        );

        let l_range = std::ops::RangeInclusive::new(
            last[0].parse::<u32>().unwrap(),
            last[1].parse::<u32>().unwrap(),
        );

        if f_range.contains(l_range.start())
            | f_range.contains(l_range.end())
            | l_range.contains(f_range.start())
            | l_range.contains(f_range.end())
        {
            result += 1;
        }
    }

    result
}
