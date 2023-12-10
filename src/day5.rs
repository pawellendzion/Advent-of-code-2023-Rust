use crate::Part;

pub struct Solution {
    part: Part,
}

impl Solution {
    pub fn new(part: Part) -> Self {
        Self { part }
    }
    
    pub fn solve(&self, puzzle_data: String) -> u64 {
        match self.part {
            Part::First => self.solve_1(puzzle_data),
            Part::Second => self.solve_2(puzzle_data)
        }
    }

    fn solve_1(&self, puzzle_data: String) -> u64 {
        let mut lines = puzzle_data.lines();
        let mut values: Vec<u64>;

        values = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut ranges: Vec<Vec<u64>> = Vec::new();

        lines.nth(1);

        loop {
            let line = lines.next();

            if line.is_none() || line.unwrap().trim().len() == 0 {
                lines.next();

                for value in &mut values {
                    for range in &ranges {
                        let destination = range[0];
                        let source = range[1];
                        let lenght = range[2];

                        if *value >= source && *value < source + lenght {
                            *value = value.abs_diff(source) + destination;
                            break;
                        }
                    }
                }
                ranges.clear();
                
                if line.is_none() {
                    break;
                } else {
                    continue;
                }
            }

            if let Some(line) = line {
                ranges.push(line.split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect()
                );
            }
        }

        values.iter().fold(u64::MAX, |acc, &x| {
            if x < acc {
                x
            } else {
               acc
            }
        })
    }

    fn solve_2(&self, puzzle_data: String) -> u64 {
        1
    }
}
