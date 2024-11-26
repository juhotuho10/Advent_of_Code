/*
- scratch cards
- card [numbers are winning numbers] | [my numbers]
part 1:
    - first match gives card 1 point and each other match doubles the point value
part 2:
    - now we win scratchcards equal to the number of winning matches we have
    - if card 10 has 5 winning matches, I get 1 card of all of the next cards, so cards 11 - 15
    - the result is the sum of the count of scratch cards
*/

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    number: u32,
    winning_nums: HashSet<u32>,
    my_nums: HashSet<u32>,
}

impl Card {
    fn new_from_string(card_str: &str) -> Self {
        let (card_num_str, all_numbers_str) = card_str.split_once(":").unwrap();
        let (_, number_str) = card_num_str.split_once(" ").unwrap();
        let number = number_str.trim().parse::<u32>().unwrap();
        let (winning_nums_str, my_nums_str) = all_numbers_str.split_once("|").unwrap();
        let winning_nums_str_vec: Vec<&str> = winning_nums_str.split(" ").collect();
        let my_nums_str_vec: Vec<&str> = my_nums_str.split(" ").collect();
        let winning_nums: Vec<u32> = winning_nums_str_vec
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let my_nums: Vec<u32> = my_nums_str_vec
            .iter()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        Card {
            number,
            winning_nums: HashSet::from_iter(winning_nums.iter().cloned()),
            my_nums: HashSet::from_iter(my_nums.iter().cloned()),
        }
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

    let example_points = get_card_points_1(&example_1);
    dbg!(&example_points);
    assert_eq!(example_points, 13);

    let my_points = get_card_points_1(_my_input);
    dbg!(&my_points);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_points = get_total_card_count_2(&example_2);
    dbg!(&example_points);
    assert_eq!(example_points, 30);

    let my_points = get_total_card_count_2(_my_input);
    dbg!(&my_points);
}

fn get_card_matches(card: &Card) -> u32 {
    let winning_nums: Vec<&u32> = card.my_nums.intersection(&card.winning_nums).collect();
    winning_nums.len() as u32
}

fn get_card_points_1(input: &[String]) -> u32 {
    let mut all_cards: Vec<Card> = vec![];
    for string in input {
        let new_card = Card::new_from_string(string);
        all_cards.push(new_card);
    }

    let mut total_points = 0;
    for card in &all_cards {
        let win_count = get_card_matches(card);
        let new_points = u32::pow(2, win_count - 1);
        total_points += new_points;
    }

    total_points
}

fn get_total_card_count_2(input: &[String]) -> u32 {
    let mut all_cards: HashMap<u32, Card> = HashMap::new();
    for string in input {
        let new_card = Card::new_from_string(string);
        all_cards.insert(new_card.number, new_card);
    }

    let mut all_cards_and_winnings: Vec<&Card> = all_cards.values().collect();

    let mut index = 0;
    loop {
        let current_card = all_cards_and_winnings.get(index);
        match current_card {
            Some(card) => {
                let win_wount = get_card_matches(card);

                let won_cards_nums: Vec<u32> =
                    ((card.number + 1)..=(card.number + win_wount)).collect();

                let won_cards: Vec<&Card> = won_cards_nums
                    .iter()
                    .filter_map(|i| all_cards.get(i))
                    .collect();

                all_cards_and_winnings.extend(won_cards);
            }
            None => break,
        }
        index += 1;
    }

    all_cards_and_winnings.len() as u32
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
