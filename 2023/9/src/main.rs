/*
we have readings of sand instability from a reader:
0 3 6 9 12 15
we have to predict the next value by first disecting it to be the difference of previous value
and do this until we have a row of 0s
0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0

and then we get the next values by adding up the lower list value and the list last value
add 0 to the last list
0 + 3 = 3
3 + 15 = 18

0   3   6   9  12  15  18
  3   3   3   3   3   3
    0   0   0   0   0

so the next number is 18
we do this to all lists and return the sum of the next values

part 2:
extrapolate backwards, so instead of trying to get a new value, we instead get the first value that should have become before the first value
and we get the sum of the predicted previous values

0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0

becomes

-3   0   3   6   9  12  15
   3   3   3   3   3   3
     0    0   0   0   0

and the value we want is -3

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Readings {
    nums: Vec<i32>,
}

impl Readings {
    fn get_vec_diff(prev_vec: &[i32]) -> Vec<i32> {
        prev_vec
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect()
    }

    fn get_prediction(&self) -> i32 {
        let mut all_vecs: Vec<Vec<i32>> = vec![self.nums.clone()];

        let mut done = false;
        loop {
            let new_diff = Readings::get_vec_diff(all_vecs.last().unwrap());

            if new_diff.iter().all(|&x| x == 0) {
                done = true;
            }

            all_vecs.push(new_diff);
            if done {
                break;
            }
        }

        let mut sum = 0;
        let mut new_values = vec![];
        for vec in all_vecs.iter().rev() {
            sum += vec.last().unwrap();
            new_values.push(sum);
        }

        sum
    }

    fn get_prev_value(&self) -> i32 {
        let mut all_vecs: Vec<Vec<i32>> = vec![self.nums.clone()];

        let mut done = false;
        loop {
            let new_diff = Readings::get_vec_diff(all_vecs.last().unwrap());

            if new_diff.iter().all(|&x| x == 0) {
                done = true;
            }

            all_vecs.push(new_diff);
            if done {
                break;
            }
        }

        let mut sum = 0;
        let mut new_values = vec![];
        for vec in all_vecs.iter().rev() {
            sum = vec.first().unwrap() - sum;
            new_values.push(sum);
        }

        sum
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

    let example_sum = get_new_number_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 114);

    let my_sum = get_new_number_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_prev_number_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 2);

    let my_sum = get_prev_number_sum_2(_my_input);
    dbg!(my_sum);
}

fn get_new_number_sum_1(input: &[String]) -> i32 {
    let readings = parse_input(input);

    let predictions: Vec<i32> = readings
        .iter()
        .map(|reading| reading.get_prediction())
        .collect();

    predictions.iter().sum()
}

fn get_prev_number_sum_2(input: &[String]) -> i32 {
    let readings = parse_input(input);

    let predictions: Vec<i32> = readings
        .iter()
        .map(|reading| reading.get_prev_value())
        .collect();

    predictions.iter().sum()
}

fn parse_input(input: &[String]) -> Vec<Readings> {
    let mut readings_vec = vec![];

    for num_string in input {
        let nums: Vec<i32> = num_string
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let new_readings = Readings { nums };

        readings_vec.push(new_readings);
    }

    readings_vec
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
