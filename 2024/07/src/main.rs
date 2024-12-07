/*
part 1:
we have a lot of equations like:
190: 10 19
and we have operations '+' and '*'
we have to find if the numbers can be solved wtih the operators
10 * 19 = 190 so this can be solved
we talke all the equations that can be solved and we sum the answers together

part 2:

we get a new operator: '||' which can concatinate the number to our existing num
*/

use doers::factorial_design::fullfact;
use ndarray::{Array2, Axis};
use num_bigint::BigUint;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Equation {
    ans: u64,
    nums: Vec<u64>,
}

impl Equation {
    fn from_string(input: &str) -> Self {
        let (ans, nums) = input.split_once(":").unwrap();

        let nums_vec: Vec<u64> = nums
            .trim()
            .split(" ")
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        Equation {
            ans: ans.parse().unwrap(),
            nums: nums_vec,
        }
    }

    fn generate_possible_combinations(&self, op_count: u16) -> Array2<u16> {
        let operator_count = self.nums.len() - 1;
        assert!(operator_count > 0);

        let possible_operations = vec![op_count; operator_count];

        fullfact(&possible_operations).unwrap()
    }

    fn fully_check_if_possible_1(&self) -> bool {
        let all_possibilities = self.generate_possible_combinations(2);

        for possibility in all_possibilities.axis_iter(Axis(0)) {
            let mut total = self.nums[0];
            for (idx_1, op) in possibility.iter().enumerate() {
                let idx = idx_1 + 1;
                match op {
                    0 => total += self.nums[idx],
                    1 => total *= self.nums[idx],
                    _ => unreachable!(),
                }
            }
            if total == self.ans {
                return true;
            }
        }
        false
    }

    fn concatenate_number(total: u64, num: u64) -> u64 {
        let mut num_string = total.to_string();
        num_string += &num.to_string();

        num_string.parse::<u64>().expect("num too large to be u64")
    }

    fn fully_check_if_possible_2(&self) -> bool {
        let all_possibilities = self.generate_possible_combinations(3);

        for possibility in all_possibilities.axis_iter(Axis(0)) {
            let mut total = self.nums[0];
            for (idx_1, op) in possibility.iter().enumerate() {
                let idx = idx_1 + 1;
                match op {
                    0 => total += self.nums[idx],
                    1 => total *= self.nums[idx],
                    2 => total = Equation::concatenate_number(total, self.nums[idx]),
                    _ => unreachable!(),
                }
            }
            if total == self.ans {
                return true;
            }
        }
        false
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

    let example_sum = valid_equation_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, BigUint::from(3749_u32));

    let my_sum = valid_equation_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = valid_equation_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, BigUint::from(11387_u32));

    let my_sum = valid_equation_sum_2(_my_input);
    dbg!(my_sum);
}

fn valid_equation_sum_1(input: &[String]) -> BigUint {
    let mut equations = parse_input(input);
    equations.retain(|eq| eq.fully_check_if_possible_1());

    let big_int_vec: Vec<BigUint> = equations.iter().map(|eq| BigUint::from(eq.ans)).collect();

    big_int_vec.iter().sum()
}

fn valid_equation_sum_2(input: &[String]) -> BigUint {
    let mut equations = parse_input(input);
    equations.retain(|eq| eq.fully_check_if_possible_2());

    let big_int_vec: Vec<BigUint> = equations.iter().map(|eq| BigUint::from(eq.ans)).collect();

    big_int_vec.iter().sum()
}

fn parse_input(input: &[String]) -> Vec<Equation> {
    input.iter().map(|s| Equation::from_string(s)).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
