/*
we have hotsprings that work or are broken
borken hotsprint = #, working hotspring = .
hotsprings are in a a line and we know how many consecutive hotsprings work or are broken
all consecutive broken hotsprings are separated by a working one
example:
.#...#....###. 1,1,3

we also have hotsprings we dont know the condition of, marked ?
we have to figure out how many different orders the broken hotsprings can be in
like:
.??..??...?##. 1,1,3

can be:

.#...#....###
..#..#....###
.#....#...###
..#...#...###
so 4 diffrent combinations
and we have to figure out the sum of all possible combinations for all hotsprings


part 2:

all springs become 5x longer, separated by ? symbol

so
.??..??...?##. 1,1,3
becomes:

.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##. 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3


*/

use fxhash::FxHashMap;
use rayon::prelude::*;
use std::fs::File;
use std::hash::{Hash, Hasher};

use std::{
    io::{BufRead, BufReader},
    iter::repeat_n,
    time::Instant,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Condition {
    Good,
    Bad,
    Idk,
}

impl Hash for Condition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Condition {
    fn fits_pattern(pattern: &[Condition], current_cond: &[Condition]) -> Option<bool> {
        let result = match (pattern, current_cond) {
            ([], []) => Some(true),
            ([], _some) => Some(false),
            (_some, []) => Some(true),
            ([Condition::Bad, ..], [Condition::Good, ..]) => None,
            (pattern, current_cond) => {
                if current_cond.len() > pattern.len() {
                    Some(false)
                } else {
                    Some(pattern.iter().zip(current_cond.iter()).all(|(p, c)| {
                        matches!(
                            (p, c),
                            (Condition::Good, Condition::Good)
                                | (Condition::Bad, Condition::Bad)
                                | (Condition::Idk, _)
                                | (_, Condition::Idk)
                        )
                    }))
                }
            }
        };

        //println!("matching pattern \n{pattern:?}\n{current_cond:?}, \nresult: {result:?}");

        result
    }
}

#[derive(Debug, Clone)]
struct Spring {
    arrangement_str: String,
    conditions: Vec<Condition>,
    broken: Vec<u8>,
}

fn option_add(option: &mut Option<u64>, value: Option<u64>) {
    if let Some(val) = value {
        match option {
            Some(prev) => *prev += val,
            None => *option = Some(val),
        }
    }
}

impl Spring {
    fn from_string_1(input: String) -> Self {
        let (arrangement_str, broken_str) = input.split_once(" ").unwrap();
        let broken: Vec<u8> = broken_str
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let conditions: Vec<Condition> = arrangement_str
            .trim_matches('.')
            .chars()
            .map(|c| match c {
                '.' => Condition::Good,
                '#' => Condition::Bad,
                '?' => Condition::Idk,
                _ => unreachable!(),
            })
            .collect();
        Spring {
            arrangement_str: arrangement_str.to_owned(),
            conditions,
            broken,
        }
    }

    fn from_string_2(input: String) -> Self {
        let (arrangement_str, broken_str) = input.split_once(" ").unwrap();

        let count_multiplier = 5;
        let arrangement_str: String = repeat_n(arrangement_str, count_multiplier)
            .collect::<Vec<_>>()
            .join("?");

        let broken_str: String = repeat_n(broken_str, count_multiplier)
            .collect::<Vec<_>>()
            .join(",");

        let broken: Vec<u8> = broken_str
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let conditions: Vec<Condition> = arrangement_str
            .chars()
            .map(|c| match c {
                '.' => Condition::Good,
                '#' => Condition::Bad,
                '?' => Condition::Idk,
                _ => unreachable!(),
            })
            .collect();
        Spring {
            arrangement_str: arrangement_str.to_owned(),
            conditions,
            broken,
        }
    }

    fn translate_combination(&self, combination: &[u32]) -> Vec<Condition> {
        let total_size: usize = combination
            .iter()
            .map(|&count| count as usize)
            .sum::<usize>()
            + self
                .broken
                .iter()
                .map(|&count| count as usize)
                .sum::<usize>();

        let mut sprint_conditions = Vec::with_capacity(total_size);

        // Add initial good conditions, this is done because there is one more good than there is bad
        sprint_conditions.resize(combination[0] as usize, Condition::Good);

        for (bad_count, good_count) in self.broken.iter().zip(&combination[1..]) {
            sprint_conditions.resize(
                sprint_conditions.len() + *bad_count as usize,
                Condition::Bad,
            );
            sprint_conditions.resize(
                sprint_conditions.len() + *good_count as usize,
                Condition::Good,
            );
        }

        sprint_conditions
    }

    fn is_valid_comb(&self, combination: &[u32]) -> bool {
        let translate_comb = self.translate_combination(combination);
        Condition::fits_pattern(&self.conditions, &translate_comb).unwrap_or(false)
    }
    fn max_possible_good(&self) -> u32 {
        // gets the max good tiles in a row
        let mut max_count = 0;
        let mut current_count = 0;

        for condition in &self.conditions {
            match condition {
                Condition::Good | Condition::Idk => {
                    current_count += 1;
                    if current_count > max_count {
                        max_count = current_count;
                    }
                }
                Condition::Bad => {
                    current_count = 0;
                }
            }
        }

        max_count
    }

    fn valid_combination_count_2(&self) -> u64 {
        // {3, 2, 1}
        // ????.???.??
        // ###..???.??
        // {2,1}
        // find next viable spot for .##. {2}
        // .???.?? not viable
        // ???.?? viable
        // {2,1}, {???.??} hashed since it always returns the same answer
        // -> going forward ->
        // ?? {2,1} no longer viable because there is no space to fit all of them
        //          so we return an error / none from this state to indicate to the last one to stop going forward
        // if we step forward from the known unknow, we return early to stop going forward since there is no viable
        // way to have the current row go forward without the result being invalid
        // for example known: #???. and known: .###. would always be invalid

        // big picture algorithm
        // 1. setup everything
        // -> start recursive function
        // 2. find first suitable spot for the first item in list
        // 3. plot down the first piece
        // 4. find the next suitable spot for the second piece
        // 5. continue recursion from that
        // 6. after that returns, just find the next suiteable spot for the current piece
        // 7. plot it down there, go back to step 4
        // continue until the 2.nd in line returns none from not having enough space,
        // meaning that going forward is not going to give results

        let mut memo: FxHashMap<(&[Condition], &[u8]), Option<u64>> = FxHashMap::default();

        let mut modified_conditions = self.conditions.clone();
        // we push good to the start and end to keep up the consistency that a valid spot starts and ends with a good condition
        // since the start can have a spot hugging the wall, we can just assume it to be good condition
        modified_conditions.insert(0, Condition::Good);
        modified_conditions.push(Condition::Good);

        let starting_point = Self::find_next_valid(&modified_conditions, &self.broken[0]).unwrap();

        Self::recursive_combination_search(1, &mut memo, starting_point, &self.broken)
            .unwrap_or_else(|| panic!("failed the current file: {:?}", self.arrangement_str))
    }

    fn recursive_combination_search<'a>(
        recursion_num: u8,
        memo: &mut FxHashMap<(&'a [Condition], &'a [u8]), Option<u64>>,
        conditions: &'a [Condition],
        broken: &'a [u8],
    ) -> Option<u64> {
        //println!("{recursion_num} funcion call with {conditions:?} and {broken:?}",);
        match (conditions, broken) {
            (_conditons_left, []) => {
                unreachable!("should be checked before entering");
            }
            ([], _some_still_broken) => {
                //println!("{recursion_num} empty!!");
                None
            }
            (mut current_cond, broken) => {
                if let Some(memo_result) = memo.get(&(current_cond, broken)) {
                    //println!("{recursion_num} memo found: {:?}", memo_result);
                    return *memo_result;
                }

                //println!("{recursion_num} matching cond and broken");

                let current_broken = &broken[0];
                let skip_dist = (current_broken + 1) as usize; // for .###.????? this skips .### onward to .?????
                let mut current_sum: Option<u64> = None;

                // No starting point, we have to return None
                loop {
                    // current broken being 3, the conditions is always guaranteed to have .###. of valid space
                    // we want to send the next iteration to start current broken + 1 later

                    //println!(
                    //    "{recursion_num} cond when staring the loop {:?}",
                    //    current_cond
                    //);

                    let next_skip = &current_cond[skip_dist..];

                    let next_val = match &broken.get(1) {
                        // we can safely go into the next recursion cycle
                        Some(next_index) => match Self::find_next_valid(next_skip, next_index) {
                            Some(next_valid_starting_point) => Self::recursive_combination_search(
                                recursion_num + 1,
                                memo,
                                next_valid_starting_point,
                                &broken[1..],
                            ),
                            None => None, // we fail before finding a new valid stop
                        },
                        // there would be no broken pieces in the next iteration
                        None => {
                            if next_skip.contains(&Condition::Bad) {
                                // the next iteration would be invalid
                                //println!("{recursion_num} next would have failed");
                                None
                            } else {
                                //println!(
                                //    "{recursion_num} +1 point with {:?} and cond NONE",
                                //    next_skip,
                                //);
                                Some(1)
                            }
                        }
                    };

                    option_add(&mut current_sum, next_val);

                    current_cond = &current_cond[1..];
                    match Self::find_next_valid(current_cond, current_broken) {
                        Some(new_cond) => current_cond = new_cond,
                        None => {
                            memo.insert((conditions, broken), current_sum);
                            return current_sum;
                        }
                    }

                    if Self::out_of_space(current_cond, broken) || current_cond.is_empty() {
                        //println!("{recursion_num} out of space");
                        memo.insert((conditions, broken), current_sum);
                        return current_sum;
                    }
                }
            }
        }
    }

    fn find_next_valid<'a>(mut conditions: &'a [Condition], num: &u8) -> Option<&'a [Condition]> {
        let wanted_condition = Self::gen_condition(num);
        while !conditions.is_empty() && !Condition::fits_pattern(conditions, &wanted_condition)? {
            conditions = &conditions[1..];
        }
        Some(conditions)
    }

    fn out_of_space(conditions: &[Condition], broken: &[u8]) -> bool {
        // sum bad condition > sum possible bad

        let available_spots = conditions
            .iter()
            .filter(|&&cond| cond != Condition::Good)
            .count();

        let wanted_spots: u8 = broken.iter().sum();
        available_spots < wanted_spots as usize
    }

    fn gen_condition(size: &u8) -> Vec<Condition> {
        // 1 -> .#.
        // 3 -> .###.

        let mut condition = vec![Condition::Good; (size + 2) as usize];
        condition[1..(*size as usize + 1)].fill(Condition::Bad);
        condition
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

    let example_arrangements = get_num_arrangements_1(&example_1);
    dbg!(&example_arrangements);
    assert_eq!(example_arrangements, 21);
    let start = Instant::now();
    let my_arrangements = get_num_arrangements_1(_my_input);
    println!("time elapsed in part 1: {}µs", start.elapsed().as_micros());
    dbg!(my_arrangements);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_arrangements = get_num_arrangements_2(&example_2);
    assert_eq!(example_arrangements, 525152);

    let start = Instant::now();
    let my_arrangements = get_num_arrangements_2(_my_input);
    println!("time elapsed in part 2: {}µs", start.elapsed().as_micros());

    dbg!(my_arrangements);
}

fn get_num_arrangements_1(input: &[String]) -> u64 {
    let springs = parse_input_1(input);

    springs
        .par_iter()
        .map(|spring| spring.valid_combination_count_2())
        .sum()
}

fn get_num_arrangements_2(input: &[String]) -> u64 {
    let springs = parse_input_2(input);

    springs
        .par_iter()
        .map(|spring| spring.valid_combination_count_2())
        .sum()
}

fn parse_input_1(input: &[String]) -> Vec<Spring> {
    input
        .iter()
        .map(|str| Spring::from_string_1(str.to_owned()))
        .collect()
}

fn parse_input_2(input: &[String]) -> Vec<Spring> {
    input
        .iter()
        .map(|str| Spring::from_string_2(str.to_owned()))
        .collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
