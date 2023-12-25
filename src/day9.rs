use std::{str::FromStr, iter::successors};

#[derive(Clone, Debug)]
struct History(Vec<i32>);

impl History {
    fn extrapolation_context(&self) -> Vec<Vec<i32>> {
        let to_diff_seq = |seq: &Vec<i32>| -> Vec<i32> {
            seq.iter()
                .zip(seq.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect()
        };

        successors(Some(to_diff_seq(&self.0)), |seq| {
            (!seq.iter().all(|e| e == &0)).then(|| to_diff_seq(seq))
        })
        .collect()
    }

    fn extrapolate_next(&self) -> i32 {
        let context = self.extrapolation_context();
        let diff = context.iter().rfold(0, |acc, x| x.last().unwrap() + acc);
        self.0.last().unwrap() + diff
    }

    fn extrapolate_prev(&self) -> i32 {
        let context = self.extrapolation_context();
        let diff = context.iter().rfold(0, |acc, x| x.first().unwrap() - acc);
        self.0.first().unwrap() - diff
    }
}

impl FromStr for History {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history: Result<Vec<i32>, _> = s
            .split_whitespace()
            .map(i32::from_str)
            .collect();

        let history = history.map_err(|err| err.to_string())?;

        if history.len() < 2 {
            return Err(String::from("Invalid str for History"));
        }

        Ok(History(history))
    }
}

struct Report(Vec<History>);

impl Report {
    fn sum_next(&self) -> i32 {
        self.0
            .iter()
            .map(History::extrapolate_next)
            .sum()
    }

    fn sum_prev(&self) -> i32 {
        self.0
            .iter()
            .map(History::extrapolate_prev)
            .sum()
    }
}

impl TryFrom<String> for Report {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let report = value
            .lines()
            .enumerate()
            .map(|(no, line)| History::from_str(line)
                 .map_err(|err| format!("Invalid history line {no}: '{line}'\n{err}"))
             )
            .collect::<Result<Vec<History>, _>>()?;

        Ok(Self(report))
    }
}

pub fn solve(input: String) -> Result<(i32, i32), String> {
    let report = Report::try_from(input)?;

    let part_1: i32 = report.sum_next();
    let part_2: i32 = report.sum_prev();

    Ok((part_1, part_2))
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn sums() -> Result<(), String>{
        let input = String::from(TEST_INPUT);
        let report = Report::try_from(input)?;
        
        println!("{:?}", report.0);

        assert_eq!(report.sum_next(), 114);
        assert_eq!(report.sum_prev(), 2);

        Ok(())
    }
}
