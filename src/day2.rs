use crate::Part;

pub struct Solution {
    part: Part,
}

impl Solution {
    pub fn new(part: Part) -> Self {
        Self { part }
    }

    pub fn solve(&self, puzzle_data: String) -> u32 {
        let mut sum = 0;

        for line in puzzle_data.lines() {
            let mut game_info = line.split(":");
            let game_no: u32 = game_info.next().unwrap().split(" ").nth(1).unwrap().parse().unwrap();
            let sets = game_info.next().unwrap().split(";");

            match self.part {
                Part::First => if self.check_if_possible(sets) { sum += game_no; },
                Part::Second => sum += self.power_of_minimum_set(sets),
            }

        }

        sum
    }

    fn check_if_possible<'a, I>(&self, sets: I) -> bool
    where 
        I: Iterator<Item = &'a str>
    {
        let counts = (12, 13, 14);

        for set in sets {
            let cubes = set.trim().split(",").map(|c| c.trim());
            for cube in cubes {
                let (count, color) = cube.split_once(" ").unwrap();
                let count: u32 = count.parse().unwrap();

                let max_count = match color {
                    "red" => counts.0 ,
                    "green" => counts.1,
                    "blue" => counts.2,
                    _ => unreachable!(),
                };

                if max_count < count {
                    return false;
                }
            }
        };

        true
    }

    fn power_of_minimum_set<'a, I>(&self, sets: I) -> u32
    where 
        I: Iterator<Item = &'a str>
    {
        let mut counts = (0, 0, 0);

        for set in sets {
            let cubes = set.trim().split(",").map(|c| c.trim());
            for cube in cubes {
                let (count, color) = cube.split_once(" ").unwrap();
                let count: u32 = count.parse().unwrap();

                let counter: &mut _ = match color {
                    "red" => &mut counts.0,
                    "green" => &mut counts.1,
                    "blue" => &mut counts.2,
                    _ => unreachable!(),
                };

                if *counter < count {
                    *counter = count;
                }
            }
        };

        counts.0 * counts.1 * counts.2
    }
}
