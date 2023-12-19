use crate::Part;

pub struct Solution {
    part: Part,
}

impl Solution {
    pub fn new(part: Part) -> Self {
        Self { part }
    }

    pub fn solve(&self, puzzle_data: String) -> u64 {
        if let Part::First = self.part {
            self.solve_1(puzzle_data)
        } else {
            self.solve_2(puzzle_data)
        }
    }

    pub fn solve_1(&self, puzzle_data: String) -> u64 {
        let mut lines = puzzle_data.lines();
        let mut times = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap());

        let mut distances = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|e| e.parse::<u64>().unwrap());

        let mut result = 1;
        while let Some(time) = times.next() {
            let dist = distances.next().unwrap();
            let (min, max) = self.hold_times(time, dist);
            result *= max.ceil() as u64 - (min as u64 + 1);
        }

        result
    }
    
    pub fn solve_2(&self, puzzle_data: String) -> u64 {
        let mut lines = puzzle_data.lines();
        let time = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .fold(0u64, |acc, e| {
                acc * 10u64.pow(e.len() as u32) + e.parse::<u64>().unwrap()
            });

        let distance = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .fold(0u64, |acc, e| {
                acc * 10u64.pow(e.len() as u32) + e.parse::<u64>().unwrap()
            });
        
        let (min, max) = self.hold_times(time, distance);
        max.ceil() as u64 - (min as u64 + 1)
    }

    fn hold_times(&self, time: u64, dist: u64) -> (f64, f64) {
        let dt = ((time.pow(2) - 4 * dist) as f64).sqrt();
        if dt > 0f64 {
            let r1 = (time as f64 - dt) / 2f64;
            let r2 = (time as f64 + dt) / 2f64;
            (r1, r2)
        } else if dt == 0f64 {
            let r0 = (time as f64 - dt) / 2f64;
            (r0, r0)
        } else {
            unreachable!()
        }
    }
}
