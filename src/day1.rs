pub fn slove(puzzle: String) -> u32 {
    struct SearchPointer(usize, Option<u32>, [usize; 9]);

    let spelled_digits = [
        "one".as_bytes(),
        "two".as_bytes(),
        "three".as_bytes(),
        "four".as_bytes(),
        "five".as_bytes(),
        "six".as_bytes(),
        "seven".as_bytes(),
        "eight".as_bytes(),
        "nine".as_bytes(),
    ];

    let mut calibration_sum = 0;

    for line in puzzle.lines() {
        let mut left = SearchPointer(0, None, [0; 9]);
        let mut right = SearchPointer(line.len() - 1, None, [0; 9]);
        let chars: Vec<_> = line.chars().collect();
        
        while (left.1.is_none() || right.1.is_none()) && left.0 <= right.0 {
            if left.1.is_none() {
                if let Some(digit) = process_char(chars[left.0], &spelled_digits, &mut left.2, false) {
                    left.1 = Some(digit);
                } else {
                    left.0 += 1;
                }
            }
            if right.1.is_none() {
                if let Some(ch) = process_char(chars[right.0], &spelled_digits, &mut right.2, true) {
                    right.1 = Some(ch);
                } else {
                    right.0 -= 1;
                }
            }
        }

        calibration_sum += left.1.unwrap_or(0) * 10 + right.1.unwrap_or(0);
    }

    calibration_sum
}

fn process_char(ch: char, spelled_digits: &[&[u8]; 9], indexes: &mut [usize ;9], reverse: bool) -> Option<u32> {
    for i in 0..9 {
        if ch.is_digit(10) {
            return ch.to_digit(10);
        }

        let spelled_digit = spelled_digits[i];
        let index = &mut indexes[i];

        if *index < spelled_digit.len() {
            let offset = if reverse { spelled_digit.len() - 1 - *index } else { *index };
            if spelled_digit[offset] == ch as u8 {
                if *index == spelled_digit.len() - 1 {
                    return Some((i as u32) + 1);
                }
                *index += 1;
            } else {
                let offset = if reverse { spelled_digit.len() - 1 } else { 0 };
                *index = if spelled_digit[offset] == ch as u8 { 1 } else { 0 };
            }
        }
    }

    None
}
