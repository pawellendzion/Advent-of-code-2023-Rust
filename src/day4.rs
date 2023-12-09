use std::collections::VecDeque;

use crate::Part;

pub struct Solution {
    part: Part,
}

impl Solution {
    pub fn new(part: Part) -> Self {
        Self { part }
    }

    pub fn solve(&self, puzzle_data: String) -> u32 {
        let lines = puzzle_data.lines();
        let mut game_points: u32 = 0;
        
        let lines_len = lines.clone().count();
        let mut copy_counter: VecDeque<u32> = VecDeque::with_capacity(lines_len);
        for _ in 0..lines_len {
            copy_counter.push_back(1);
        }

        for line in lines {
            let mut points = 0;

            let (wining_nums, have_nums) = line
                .split_once(':').unwrap().1
                .split_once('|').unwrap();
            
            let wining_nums: Vec<&str> = wining_nums
                .split(' ')
                .filter(|e| e.len() > 0)
                .collect();
            
            let have_nums = have_nums
                .split(' ')
                .filter(|e| e.len() > 0);

            for num in have_nums {
                let win_num = wining_nums.iter().find(|&&n| n == num);
                if let Some(_) = win_num {
                    if let Part::First = self.part {
                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                    } else {
                        points += 1;
                        let current_copies = copy_counter[0];
                        if let Some(counter) = copy_counter.get_mut(points as usize) {
                            *counter += current_copies;
                        }
                    }
                }
            }

            if let Part::First = self.part {
                game_points += points;
            } else {
                game_points += copy_counter.pop_front().unwrap();
            }
        }
        
        game_points
    }
}
