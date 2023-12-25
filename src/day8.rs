use std::{collections::HashMap, cmp::{PartialEq, PartialOrd}, ops::{Sub, Mul, Div}};

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("Invalid instruction {value}"))
        }
    }
}

struct Map {
    instructions: Vec<Direction>,
    nodes_connections: HashMap<String, (String, String)>
}

impl TryFrom<String> for Map {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut lines = s.lines();
        
        let instructions: Result<Vec<Direction>, _> = lines
            .next()
            .ok_or_else(|| String::from("Instructions line not found"))?
            .chars()
            .map(|ins| Direction::try_from(ins))
            .collect();

        let instructions = instructions?;
        let mut nodes_connections = HashMap::new();

        for line in lines.skip(1) {
            if line.len() != 16 {
                return Err(format!("Invalid node connection signature, expected XXX = (XXX, XXX) got {line}"));
            }

            nodes_connections.insert(
                line[0..3].to_string(), 
                (line[7..10].to_string(), line[12..15].to_string())
            );
        }

        Ok(Self { instructions, nodes_connections })
    }
}

struct Solution<'a> {
    map: &'a Map
}

impl<'a> Solution<'a> {
    fn with_map(map: &'a Map) -> Self {
        Self { map }
    }

    fn part_1(&self) -> Result<u64, String> {
        self.solve("AAA", |node| node == "ZZZ")
    }

    fn part_2(&self) -> Result<u64, String> {
        self.map.nodes_connections
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|start| self.solve(start, |node| node.ends_with('Z')))
            .try_fold(1, |acc, steps| Ok(self.lcm(acc, steps?)))
    }

    fn solve<F>(&self, start_node: &str, is_end_node: F) -> Result<u64, String>
    where F: Fn(&str) -> bool {
        let mut steps = 0;
        let mut current = start_node;
        let mut instructions = self.map.instructions.iter().cycle();

        while !is_end_node(current) {
            let Some(ins) = instructions.next() else {
                return Err(String::from("Cannot read instruction"));
            };

            let Some((left, right)) = self.map.nodes_connections.get(current) else {
                return Err(format!("Cannot find node {current}"));
            };

            current = match ins {
                Direction::Left => left,
                Direction::Right => right,
            };

            steps += 1;
        }

        Ok(steps)
    } 

    fn lcm<T>(&self, a: T, b: T) -> T 
    where T: PartialEq + PartialOrd + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Copy {
        let gcd = |mut a: T, mut b: T| -> T {
            while a != b {
                if a > b {
                    a = a - b;
                } else {
                    b = b - a;
                }
            }

            a
        };

        (a * b) / gcd(a, b)
    }
}

pub fn solve(puzzle_data: String) -> Result<(u64, u64), String> {
    let map = Map::try_from(puzzle_data)?;
    let solution = Solution::with_map(&map);

    let part_1 = solution.part_1()?;
    let part_2 = solution.part_2()?;

    Ok((part_1, part_2))
}
