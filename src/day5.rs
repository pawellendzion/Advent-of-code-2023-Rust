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
        let mut values: Vec<u64> = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut maps: Vec<Vec<u64>> = Vec::new();

        lines.nth(1);

        loop {
            let line = lines.next();

            if line.is_none() || line.unwrap().trim().len() == 0 {
                lines.next();

                for value in values.iter_mut() {
                    for map in maps.iter() {
                        let map_destination = map[0];
                        let map_start = map[1];
                        let map_end = map_start + map[2];

                        if *value >= map_start && *value < map_end {
                            *value = value.abs_diff(map_start) + map_destination;
                            break;
                        }
                    }
                }

                maps.clear();
                
                if line.is_none() {
                    break;
                } else {
                    continue;
                }
            }

            if let Some(line) = line {
                maps.push(line.split_whitespace()
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
        let mut lines = puzzle_data.lines();

        let mut value_ranges: Vec<(u64, u64)> = Vec::new();
        let mut values = lines
            .next().unwrap()
            .split_once(':').unwrap().1
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap());

        while let Some(start) = values.next() {
            let length = values.next().unwrap();
            value_ranges.push((start, start + length));
        }
        
        let mut maps: Vec<Vec<u64>> = Vec::new();

        lines.nth(1);
        
        loop {
            let line = lines.next();

            if line.is_none() || line.unwrap().trim().len() == 0 {
                lines.next();

                let mut buffer: Vec<(u64, u64)> = Vec::new();

                for value_range in value_ranges.iter_mut() {
                    let vr_start = value_range.0;
                    let vr_end = value_range.1;

                    let mut min_start = vr_end;
                    let mut max_end = vr_start;

                    for map in maps.iter() {
                        let map_destination = map[0];
                        let map_start = map[1];
                        let map_end = map_start + map[2];
                        
                        if !(map_end <= vr_start || vr_end <= map_start) {
                            let mut new_start = vr_start.max(map_start);
                            let mut new_end = vr_end.min(map_end);

                            if new_start < min_start {
                                min_start = new_start;
                            }

                            if new_end > max_end {
                                max_end = new_end;
                            }

                            let offset = new_start.abs_diff(map_start);
                            let len = new_start.abs_diff(new_end);

                            new_start = offset + map_destination;
                            new_end = new_start + len;

                            buffer.push((new_start, new_end));
                        }
                    }
                    
                    if min_start < vr_end && min_start > vr_start {
                        buffer.push((vr_start, min_start));
                    }

                    if max_end > vr_start && max_end < vr_end {
                        buffer.push((max_end, vr_end));
                    }

                    if min_start == vr_end && max_end == vr_start {
                        buffer.push((vr_start, vr_end));
                    }
                }

                value_ranges = buffer;
                maps.clear();
                
                if line.is_none() {
                    break;
                } else {
                    continue;
                }
            }

            if let Some(line) = line {
                maps.push(line.split_whitespace()
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect()
                );
            }
        }

        value_ranges.iter().fold(u64::MAX, |acc, &x| {
            if x.0 < acc {
                x.0
            } else {
               acc
            }
        })
    }
}
