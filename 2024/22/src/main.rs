/*
part 1:
we sell hiding spots in order to buy bananas with them.
we have to pick the perfect time to see in order to get the most bananas.
we have a secret number that we use to derive other secret nubmers from using a formula.
we can then use these secret numbers to predict prices

we get the 2000th number for each initial number and sum them  up

part 2:

the price is the number at the first position in the secret number.
monkey will sell only after a specific sequence of changes.

we have to find the best 4 len difference sequence in the price to tell the monkey that gives us the most bananas

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

struct SecretNum {
    num: i64,
    price: Vec<u8>,
    diff: Vec<i8>,
}

impl SecretNum {
    fn from_string(input: &str) -> Self {
        SecretNum {
            num: input.parse().unwrap(),
            price: vec![],
            diff: vec![],
        }
    }

    fn recursive_secret_num_finder(&mut self, mut current: i64, counter: u32) -> i64 {
        let price = (current % 10) as u8;

        if let Some(&last_price) = self.price.last() {
            let diff = price as i8 - last_price as i8;
            self.diff.push(diff);
        } else {
            self.diff.push(0);
        }

        self.price.push(price);
        if counter == 0 {
            return current;
        }

        let mix_num = current * 64;
        Self::mix(&mut current, mix_num);
        Self::prune(&mut current);

        let mix_num_2 = (current as f64 / 32.0).floor() as i64;
        Self::mix(&mut current, mix_num_2);
        Self::prune(&mut current);

        let mix_num_3 = current * 2048;
        Self::mix(&mut current, mix_num_3);
        Self::prune(&mut current);

        self.recursive_secret_num_finder(current, counter - 1)
    }

    fn mix(current: &mut i64, value: i64) {
        *current ^= value;
    }

    fn prune(current: &mut i64) {
        *current %= 16777216;
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

    let example_sum = secret_number_finder(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 37327623);

    let my_sum = secret_number_finder(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = find_best_buying_sequence(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 23);

    let my_sum = find_best_buying_sequence(_my_input);
    dbg!(my_sum);
}

fn secret_number_finder(input: &[String]) -> i64 {
    let mut secret_nums = parse_input(input);

    let mut total = 0;

    for secret in secret_nums.iter_mut() {
        let result = secret.recursive_secret_num_finder(secret.num, 2000);

        total += result;
    }

    total
}

fn find_best_buying_sequence(input: &[String]) -> u32 {
    let mut secret_nums = parse_input(input);

    for secret in secret_nums.iter_mut() {
        secret.recursive_secret_num_finder(secret.num, 2000);
    }

    let mut best_banana_count: u32 = 0;

    // inneficcient but works well for this scenario
    for diff_1 in -9..=9 {
        for diff_2 in -9..=9 {
            for diff_3 in -9..=9 {
                for diff_4 in -9..=9 {
                    let diffs = [diff_1, diff_2, diff_3, diff_4];

                    let mut current_bananas: u32 = 0;

                    for secret in &secret_nums {
                        for (i, diff_seq) in secret.diff.windows(4).enumerate() {
                            if diff_seq == diffs {
                                let index = i + 3;
                                let current_price = secret.price[index];
                                current_bananas += current_price as u32;
                                break;
                            }
                        }
                    }

                    best_banana_count = best_banana_count.max(current_bananas)
                }
            }
        }
    }

    best_banana_count
}

fn parse_input(input: &[String]) -> Vec<SecretNum> {
    input.iter().map(|s| SecretNum::from_string(s)).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
