/*
we have a list of decks and the points that they have commited to the game
we have to order the decks based on poker decks
if 2 hands have the same poker hand, we instead compare the numbers from left to right
for each deck we multiply the ranking with the score
so for 1000 hands, the best scoring hand gets 1000 * points, all the way to worst hand getting 1 * points
and we sum up the points for all the decks

part2:
J is now a joker card and it's value is now the lowest
but the Joker card can act as any other card making it so that we get the highet hand type possible easily

*/

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    House,
    Four,
    Five,
}

#[derive(Debug, Eq)]
struct Hand {
    _cards: String,
    cards_hex: u64,
    hand_type: HandType,
    points_commited: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // first we compare the hand type, higher hand type wins by default
        match self.hand_type.cmp(&other.hand_type) {
            // if the hands are equal, we compare by the hand cards from left to right
            // represented by the hex of the hand
            Ordering::Equal => self.cards_hex.cmp(&other.cards_hex),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards_hex == other.cards_hex
    }
}

impl Hand {
    fn from_string_1(input: String) -> Self {
        let (hand_str, points_str) = input.split_once(" ").unwrap();
        let points: u32 = points_str.parse::<u32>().unwrap();

        Hand {
            _cards: hand_str.to_owned().clone(),
            cards_hex: Hand::get_hand_hex_1(hand_str),
            hand_type: Hand::get_hand_type_1(hand_str),
            points_commited: points,
        }
    }

    fn get_hand_type_1(input: &str) -> HandType {
        let cards: Vec<char> = input.chars().collect();

        let mut char_counts = HashMap::new();

        for c in &cards {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        let max_count = char_counts.values().max().unwrap();

        match max_count {
            5 => HandType::Five,
            4 => HandType::Four,
            3 => {
                let num_2_pairs: usize = char_counts.values().filter(|&&count| count == 2).count();

                match num_2_pairs {
                    1 => HandType::House,
                    0 => HandType::Three,
                    _ => unreachable!(),
                }
            }
            2 => {
                let num_2_pairs: usize = char_counts.values().filter(|&&count| count == 2).count();
                match num_2_pairs {
                    2 => HandType::TwoPair,
                    1 => HandType::Pair,
                    _ => unreachable!(),
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }

    fn get_hand_hex_1(input: &str) -> u64 {
        let card_to_hex_vec = vec![
            ('2', '1'),
            ('3', '2'),
            ('4', '3'),
            ('5', '4'),
            ('6', '5'),
            ('7', '6'),
            ('8', '7'),
            ('9', '8'),
            ('T', '9'),
            ('J', 'A'),
            ('Q', 'B'),
            ('K', 'C'),
            ('A', 'D'),
        ];

        let conversion_table: HashMap<char, char> = card_to_hex_vec.into_iter().collect();

        let converted_string: String = input
            .chars()
            .map(|ch| conversion_table.get(&ch).copied().unwrap())
            .collect();

        u64::from_str_radix(&converted_string, 16).unwrap()
    }

    fn from_string_2(input: String) -> Self {
        let (hand_str, points_str) = input.split_once(" ").unwrap();
        let points: u32 = points_str.parse::<u32>().unwrap();

        Hand {
            _cards: hand_str.to_owned().clone(),
            cards_hex: Hand::get_hand_hex_2(hand_str),
            hand_type: Hand::get_hand_type_2(hand_str),
            points_commited: points,
        }
    }

    fn get_hand_type_2(input: &str) -> HandType {
        let mut all_possible_hands: Vec<String> = vec![];

        for c in [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ] {
            // get all possible combinations for what J could be
            let new_string = input.replace('J', &c.to_string());
            all_possible_hands.push(new_string);
        }

        let mut all_hand_types: Vec<HandType> = vec![];

        for input_possibility in all_possible_hands {
            let cards: Vec<char> = input_possibility.chars().collect();

            let mut char_counts = HashMap::new();

            for c in &cards {
                *char_counts.entry(c).or_insert(0) += 1;
            }

            let max_count = char_counts.values().max().unwrap();

            let current_hand_type = match max_count {
                5 => HandType::Five,
                4 => HandType::Four,
                3 => {
                    let num_2_pairs: usize =
                        char_counts.values().filter(|&&count| count == 2).count();

                    match num_2_pairs {
                        1 => HandType::House,
                        0 => HandType::Three,
                        _ => unreachable!(),
                    }
                }
                2 => {
                    let num_2_pairs: usize =
                        char_counts.values().filter(|&&count| count == 2).count();
                    match num_2_pairs {
                        2 => HandType::TwoPair,
                        1 => HandType::Pair,
                        _ => unreachable!(),
                    }
                }
                1 => HandType::HighCard,
                _ => unreachable!(),
            };

            all_hand_types.push(current_hand_type);
        }

        // get the max hand type it could possibly be
        *all_hand_types.iter().max().unwrap()
    }

    fn get_hand_hex_2(input: &str) -> u64 {
        let card_to_hex_vec = vec![
            ('J', '1'),
            ('2', '2'),
            ('3', '3'),
            ('4', '4'),
            ('5', '5'),
            ('6', '6'),
            ('7', '7'),
            ('8', '8'),
            ('9', '9'),
            ('T', 'A'),
            ('Q', 'B'),
            ('K', 'C'),
            ('A', 'D'),
        ];

        let conversion_table: HashMap<char, char> = card_to_hex_vec.into_iter().collect();

        let converted_string: String = input
            .chars()
            .map(|ch| conversion_table.get(&ch).copied().unwrap())
            .collect();

        u64::from_str_radix(&converted_string, 16).unwrap()
    }
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_points = get_total_points_1(&example_1);
    dbg!(example_points);
    assert_eq!(example_points, 6440);

    let my_points = get_total_points_1(_my_input);
    dbg!(my_points);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_points = get_total_points_2(&example_2);
    dbg!(example_points);
    assert_eq!(example_points, 5905);

    let my_points = get_total_points_2(_my_input);
    dbg!(my_points);
}

fn get_total_points_1(input: &[String]) -> u64 {
    let mut hands_vec: Vec<Hand> = input
        .iter()
        .map(|s| Hand::from_string_1(s.to_string()))
        .collect();

    // sort the hand deck by the ordering that we have defined for the Hand type
    hands_vec.sort();

    let mut total_points: u64 = 0;
    for (i, hand) in hands_vec.iter().enumerate() {
        let multiplied_points = (i + 1) as u64 * hand.points_commited as u64;
        total_points += multiplied_points;
    }

    total_points
}

fn get_total_points_2(input: &[String]) -> u64 {
    let mut hands_vec: Vec<Hand> = input
        .iter()
        .map(|s| Hand::from_string_2(s.to_string()))
        .collect();

    // sort the hand deck by the ordering that we have defined for the Hand type
    hands_vec.sort();

    let mut total_points: u64 = 0;
    for (i, hand) in hands_vec.iter().enumerate() {
        let multiplied_points = (i + 1) as u64 * hand.points_commited as u64;
        total_points += multiplied_points;
    }

    total_points
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
