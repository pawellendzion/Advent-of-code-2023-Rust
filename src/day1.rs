pub fn slove(puzzle: String) -> u32 {
    let mut calibration_sum = 0;

    for line in puzzle.lines() {
        let mut left = (0, None);
        let mut right = (line.len() - 1, None);
        let chars: Vec<_> = line.chars().collect();

        while (left.1.is_none() || right.1.is_none()) && left.0 <= right.0 {
            if left.1.is_none() {
                if chars[left.0].is_digit(10) {
                    left.1 = Some(chars[left.0]);
                } else {
                    left.0 += 1;
                }
            }
            if right.1.is_none() {
                if chars[right.0].is_digit(10) {
                    right.1 = Some(chars[right.0]);
                } else {
                    right.0 -= 1;
                }
            }
        }
        
        calibration_sum += 
            left.1.unwrap_or('0').to_digit(10).unwrap_or(0) * 10 +
            right.1.unwrap_or('0').to_digit(10).unwrap_or(0);
    }

    calibration_sum
}
