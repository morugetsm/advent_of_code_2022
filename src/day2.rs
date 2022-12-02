use std::cmp::Ordering;

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl std::cmp::PartialEq for RPS {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Rock, Self::Rock) => true,
            (Self::Paper, Self::Paper) => true,
            (Self::Scissors, Self::Scissors) => true,
            _ => false,
        }
    }
}

impl std::cmp::PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Rock, Self::Rock) => Some(Ordering::Equal),
            (Self::Rock, Self::Paper) => Some(Ordering::Less),
            (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
            (Self::Paper, Self::Rock) => Some(Ordering::Greater),
            (Self::Paper, Self::Paper) => Some(Ordering::Equal),
            (Self::Paper, Self::Scissors) => Some(Ordering::Less),
            (Self::Scissors, Self::Rock) => Some(Ordering::Less),
            (Self::Scissors, Self::Paper) => Some(Ordering::Greater),
            (Self::Scissors, Self::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl From<&str> for RPS {
    fn from(keyword: &str) -> Self {
        match keyword {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!(),
        }
    }
}

// misunderstand of question but i got the answer
pub fn part1() -> u32 {
    let input = std::fs::read_to_string("./input/day2.txt").unwrap();

    let mapping_table = [
        [RPS::Rock, RPS::Paper, RPS::Scissors],
        // [RPS::Rock, RPS::Scissors, RPS::Paper],
        // [RPS::Paper, RPS::Rock, RPS::Scissors],
        // [RPS::Paper, RPS::Scissors, RPS::Rock],
        // [RPS::Scissors, RPS::Rock, RPS::Paper],
        // [RPS::Scissors, RPS::Paper, RPS::Rock],
    ];

    let mut predicts = Vec::new();

    mapping_table.iter().for_each(|table| {
        let (your_score, my_score) = input.lines().fold((0, 0), |(your_acc, my_acc), line| {
            let mut vs = line.split_whitespace();

            let first = vs.next().unwrap();
            let last = vs.next().unwrap();

            let you = RPS::from(first);
            let me = match last {
                "X" => table.get(0).unwrap(),
                "Y" => table.get(1).unwrap(),
                "Z" => table.get(2).unwrap(),
                _ => panic!(),
            };

            let your_select = match you {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            };

            let your = match &you.partial_cmp(me).unwrap() {
                Ordering::Greater => 6 + your_select,
                Ordering::Equal => 3 + your_select,
                Ordering::Less => 0 + your_select,
            };

            let my_select = match me {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            };

            let my = match me.partial_cmp(&you).unwrap() {
                Ordering::Greater => 6 + my_select,
                Ordering::Equal => 3 + my_select,
                Ordering::Less => 0 + my_select,
            };

            (your_acc + your, my_acc + my)
        });

        if my_score > your_score {
            predicts.push(my_score);
        }
    });

    println!("{:?}", predicts);
    *predicts.iter().min().unwrap()
}

pub fn part2() -> u32 {
    let input = std::fs::read_to_string("./input/day2.txt").unwrap();
    
    input.lines().fold(0, |acc, line| {
        let mut vs = line.split_whitespace();

        let first = vs.next().unwrap();
        let last = vs.next().unwrap();

        let you = RPS::from(first);
        let me = match (&you, last) {
            (RPS::Rock, "X") => RPS::Scissors,
            (RPS::Rock, "Y") => RPS::Rock,
            (RPS::Rock, "Z") => RPS::Paper,
            (RPS::Paper, "X") => RPS::Rock,
            (RPS::Paper, "Y") => RPS::Paper,
            (RPS::Paper, "Z") => RPS::Scissors,
            (RPS::Scissors, "X") => RPS::Paper,
            (RPS::Scissors, "Y") => RPS::Scissors,
            (RPS::Scissors, "Z") => RPS::Rock,
            _ => panic!(),
        };

        let my_select = match me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        let my = match me.partial_cmp(&you).unwrap() {
            Ordering::Greater => 6 + my_select,
            Ordering::Equal => 3 + my_select,
            Ordering::Less => 0 + my_select,
        };

        acc + my
    })
}
