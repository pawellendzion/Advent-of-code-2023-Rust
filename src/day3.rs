use std::collections::HashMap;

use crate::Part;

struct CharPos {
    byte: u8,
    row: usize,
    col: usize,
}

impl CharPos {
    pub fn new(byte: u8, row: usize, col: usize) -> Self {
        Self { byte, row, col }
    }
}

pub struct Solution {
    part: Part,
}

impl Solution {
    pub fn new(part: Part) -> Self {
        Self { part }
    }

    pub fn solve(&self, puzzle_data: String) -> u32 {
        let mut sum = 0;
        let lines: Vec<&str> = puzzle_data.lines().collect();

        let mut lines_buf: [(usize, &[u8]); 3] = [
            (0, &[]), (0, &[]),
            (0, lines[0].as_bytes()),
        ];

        let mut multiply_sets: HashMap<String, Vec<u32>> = HashMap::new();
        for ri in 0..lines.len() {
            lines_buf[0] = lines_buf[1];
            lines_buf[1] = lines_buf[2];
            lines_buf[2] = if ri < lines.len() - 1 {
                (ri + 1, lines[ri + 1].as_bytes())
            } else {
                (0, &[])
            };

            let mut adjected = (false, CharPos::new(0, 0, 0));
            let mut number: u32 = 0;
            for ci in 0..lines_buf[1].1.len() {
                if lines_buf[1].1[ci].is_ascii_digit() {
                    number = number * 10 + (lines_buf[1].1[ci] as u32) - 48;

                    if number < 10 && ci > 0 {
                        adjected = self.check_if_adjected(&lines_buf, ci - 1);
                    }

                    if !adjected.0 { 
                        adjected = self.check_if_adjected(&lines_buf, ci);
                    }

                    if adjected.0 && ci == lines_buf[1].1.len() - 1 {
                        if let Part::First = self.part {
                            sum += number;
                        } else if adjected.1.byte == b'*' {
                            self.insert_to_hash_map(&mut multiply_sets, &adjected.1, number);
                        }
                    }
                } else if adjected.0 {
                    if let Part::First = self.part {
                        sum += number;
                    } else if adjected.1.byte == b'*' {
                        self.insert_to_hash_map(&mut multiply_sets, &adjected.1, number);
                    }
                    adjected = (false, CharPos::new(0, 0, 0));
                    number = 0;
                } else if number > 0 {
                    adjected = self.check_if_adjected(&lines_buf, ci);
                    if adjected.0 {
                        if let Part::First = self.part {
                            sum += number;
                        } else if adjected.1.byte == b'*' {
                            self.insert_to_hash_map(&mut multiply_sets, &adjected.1, number);
                        }
                    }

                    number = 0;
                    adjected = (false, CharPos::new(0, 0, 0));
                }
            }
        }
        
        if let Part::First = self.part {
            sum
        } else {
            multiply_sets
                .values()
                .filter(|e| e.len() > 1)
                .map(|e| e.into_iter()
                     .fold(1, |acc, &x| acc * x )
                )
                .sum()
        }
    }

    fn check_if_adjected(&self, lines_buf: &[(usize, &[u8]); 3], col_index: usize) -> (bool, CharPos) {
        for i in 0..3 {
            if lines_buf[i].1.len() != 0 &&
                !lines_buf[i].1[col_index].is_ascii_digit() &&
                lines_buf[i].1[col_index] != '.' as u8 
            {
                return (true, CharPos::new(lines_buf[i].1[col_index], lines_buf[i].0, col_index));
            }
        }

        return (false, CharPos::new(0, 0, 0));
    }

    fn insert_to_hash_map(&self, hash_map: &mut HashMap<String, Vec<u32>>, char_pos: &CharPos, number: u32) {
        let key = format!("{} {}", char_pos.row, char_pos.col);
        if let Some(nums) = hash_map.get_mut(&key) {
            nums.push(number);
        } else {
            hash_map.insert(key, vec![number]);
        }
    }
}
