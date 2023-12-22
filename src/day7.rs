use std::collections::HashMap;
use std::cmp::Ordering;

use crate::Part;

pub struct Solution {
    part: Part,
    card_weights: HashMap<char, i32>
}

impl Solution {
    pub fn new(part: Part) -> Self {
        let mut card_weights = HashMap::from([
            ('2', 2), ('3', 3), ('4', 4),
            ('5', 5), ('6', 6), ('7', 7),
            ('8', 8), ('9', 9), ('T', 10),
            ('Q', 12), ('K', 13), ('A', 14),
        ]);

        if let Part::First = part {
            card_weights.insert('J', 11);
        } else {
            card_weights.insert('J', 1);
        }

        Self { part, card_weights }
    }

    pub fn solve(&self, puzzle_data: String) -> u32 {
        let mut lines = puzzle_data.lines();
        let mut hand_types: [Vec<(&str, u32)>; 7] = Default::default();
        let mut result = 0;

        while let Some(line) = lines.next() {
            let (hand, bid) = line.split_once(' ').unwrap();

            let mut wildcard_card = (None, 0);
            let mut hand_type = 0;
            let mut cards_occurrances = HashMap::with_capacity(13);

            for card in hand.chars() {
                let occurrances = match cards_occurrances.get_mut(&card) {
                    Some(occurrances) => occurrances,
                    None => {
                        cards_occurrances.insert(card, 0);
                        cards_occurrances.get_mut(&card).unwrap()
                    },
                };

                *occurrances += 1;

                if let Part::Second = self.part {
                    if card == 'J' {
                        continue;
                    }
                }

                if *occurrances > wildcard_card.1 {
                    wildcard_card = (Some(card), *occurrances);
                }

                hand_type += *occurrances;
            }

            if let Part::Second = self.part {
                if let Some(j_card) = cards_occurrances.get(&'J') {
                    let occurrances = if let Some(card_type) = wildcard_card.0 {
                        cards_occurrances.get(&card_type).unwrap_or(&0)
                    } else { &0 };

                    for i in *occurrances + 1..*occurrances + 1 + j_card {
                        hand_type += i
                    }
                }
            }

            let type_index = match hand_type {
                5 => 0,
                6 => 1,
                7 => 2,
                8 => 3,
                9 => 4,
                11 => 5,
                15 => 6,
                _ => unreachable!("{hand} {hand_type}")
            };

            hand_types[type_index].push((hand, bid.parse().unwrap()));
        }

        let mut rank = 1;
        for hand_type in hand_types.iter_mut() {
            hand_type.sort_by(|a,b| {
                let mut a_chs = a.0.chars();
                let mut b_chs = b.0.chars();

                let mut i = 0;
                loop {
                    let ai_weight = self.card_weights.get(&a_chs.next().unwrap()).unwrap();
                    let bi_weight = self.card_weights.get(&b_chs.next().unwrap()).unwrap();

                    let compare = ai_weight.cmp(bi_weight);

                    if i == 4 {
                        return compare;
                    }

                    if let Ordering::Equal = compare {
                        i += 1;
                        continue;
                    }

                    return compare;
                }
            });

            for (_, bid) in hand_type {
                result += *bid * rank;
                rank += 1;
            }
        }

        result
    }
}
