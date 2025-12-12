/*
part 1:
in the input we have light diagrams button wiring schematics and joltage requirements for each machine
each combination turns on and turns off different light
we have to find the combinations of lights to press in a way where we press the least amount of buttons to get to the
wanted light combination

part 2:
now we have to take the joltage requirements into account
now pressing the buttons adds 1 to the joltage of the machine whereever the buttons are wired
we have to find the fewest button presses to get to the wanted joltages for each machine

*/

use ahash::AHashMap;
use itertools::{Combinations, Itertools};
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use nalgebra::{DMatrix, DVector, LU};

struct MachineRequirements {
    lights_requirement: u16,
    wiring_bits: Vec<u16>,
    wiring_vec: Vec<Vec<u8>>,
    wiring_vec_index: Vec<Vec<bool>>,
    joltage_requirements: Vec<u16>,
}

impl MachineRequirements {
    #[allow(clippy::ptr_arg)]
    fn new(input: &String) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();

        assert!(parts.len() >= 3);
        let [lights_str, buttons_str @ .., joltages_str] = parts.as_slice() else {
            unreachable!()
        };

        let lights: u16 = lights_str
            .chars()
            .skip(1)
            .enumerate()
            .fold(0u16, |acc, (i, c)| {
                let bit = match c {
                    '#' => 1u16,
                    '.' => 0u16,
                    _ => return acc,
                };
                acc | (bit << i)
            });

        let digit_re = Regex::new(r"\d+").unwrap();

        let button_vecs: Vec<Vec<u8>> = buttons_str
            .iter()
            .map(|buttons_str| {
                digit_re
                    .find_iter(buttons_str)
                    .filter_map(|d| d.as_str().parse::<u8>().ok())
                    .collect()
            })
            .collect();

        let mut buttons: Vec<u16> = Vec::new();

        for nums_row in &button_vecs {
            let mut new_vec = 0;
            for i in nums_row.iter().rev() {
                new_vec |= 1 << i;
            }
            buttons.push(new_vec);
        }

        let joltages: Vec<u16> = digit_re
            .find_iter(joltages_str)
            .map(|d| d.as_str().parse::<u16>().unwrap())
            .collect();

        let mut button_vecs_index = Vec::new();

        for btn_vec in &button_vecs {
            let mut empty: Vec<bool> = joltages.iter().map(|_| false).collect();
            for i in btn_vec {
                empty[*i as usize] = true;
            }
            button_vecs_index.push(empty);
        }

        MachineRequirements {
            lights_requirement: lights,
            wiring_bits: buttons,
            wiring_vec: button_vecs,
            wiring_vec_index: button_vecs_index,
            joltage_requirements: joltages,
        }
    }

    fn solve_requirements(self) -> u16 {
        let mut memo: AHashMap<u16, u16> = AHashMap::new();
        self.requirements_recursive(&mut memo, &self.wiring_bits, 0, 1)
            .unwrap()
    }

    fn requirements_recursive(
        &self,
        memo: &mut AHashMap<u16, u16>,
        available_wiring_bits: &[u16],
        current_btns: u16,
        cost: u16,
    ) -> Option<u16> {
        if let Some(prev_cost) = memo.get_mut(&current_btns) {
            if *prev_cost <= cost {
                return None;
            } else {
                *prev_cost = cost;
            }
        } else {
            memo.insert(current_btns, cost);
        }

        let found_answer = available_wiring_bits
            .iter()
            .any(|wiring| (*wiring ^ current_btns) == self.lights_requirement);

        if found_answer {
            Some(cost)
        } else {
            available_wiring_bits
                .iter()
                .enumerate()
                .filter_map(|(i, wiring)| {
                    self.requirements_recursive(
                        memo,
                        &available_wiring_bits[i..],
                        *wiring ^ current_btns,
                        cost + 1,
                    )
                })
                .min()
        }
    }

    fn solve_joltages(self) -> u16 {
        let mut current_joltages: Vec<u16> = self.joltage_requirements.iter().map(|_| 0).collect();
        dbg!(self.least_sqaure_joltages());

        //self.joltages_recursive(&self.wiring_vec, &mut current_joltages, 0)
        //    .unwrap()
        11
    }

    fn least_sqaure_joltages(&self) -> Option<u16> {
        // todo: remove all paths that are just combinations of other paths and
        // add them back at the end by subtracting the sub parts to become the combination
        let mut dedup_wirings: Vec<Vec<bool>> = Vec::new();
        let mut copy_wires: Vec<(usize, Vec<usize>)> = Vec::new();

        for i in 0..self.wiring_vec_index.len() {
            let mut wiring_i: Vec<usize> = (0..self.wiring_vec_index.len()).collect();
            let current_i = wiring_i.remove(i);
            let current_wire = &self.wiring_vec_index[current_i];
            let mut is_duplicate = false;
            for n_comb in 2..wiring_i.len() {
                if is_duplicate {
                    break;
                }
                for comb in wiring_i.iter().combinations(n_comb) {
                    if is_duplicate {
                        break;
                    }
                    let comb_vecs: Vec<&Vec<bool>> =
                        comb.iter().map(|i| &self.wiring_vec_index[**i]).collect();
                    let comb_true_count = comb_vecs
                        .iter()
                        .flat_map(|vec| vec.iter())
                        .filter(|bool| **bool)
                        .count();
                    let current_true_count = current_wire.iter().filter(|bool| **bool).count();
                    if comb_true_count != current_true_count {
                        continue;
                    }

                    let mut combined: Vec<bool> = current_wire.iter().map(|_| false).collect();
                    for current_comb in comb_vecs {
                        for (i, b) in current_comb.iter().enumerate() {
                            if *b {
                                combined[i] = true;
                            }
                        }
                    }

                    if combined == *current_wire {
                        is_duplicate = true;
                        copy_wires.push((current_i, comb.into_iter().copied().collect()));
                    }
                }
            }

            if !is_duplicate {
                dedup_wirings.push(current_wire.clone());
            }
        }

        //dbg!(&dedup_wirings);
        dbg!(&!copy_wires.is_empty());

        let n_joltages = self.joltage_requirements.len();
        let n_pats = dedup_wirings.len();

        let flat_wirings: Vec<f32> = dedup_wirings
            .iter()
            .flatten()
            .copied()
            .map(|b| if b { 1.0 } else { 0.0 })
            .collect();
        let paths_array = DMatrix::from_row_slice(n_pats, n_joltages, &flat_wirings).transpose();
        let joltages_array = DVector::from_row_slice(&self.joltage_requirements).map(|x| x as f32);

        MachineRequirements::least_squares(paths_array, joltages_array)
            .map(|matrix| matrix.sum() as u16)
    }

    fn joltages_recursive(
        &self,
        available_wiring_vecs: &[Vec<u8>],
        current_joltages: &mut [u16],
        cost: u16,
    ) -> Option<u16> {
        assert!(current_joltages.len() == self.joltage_requirements.len());
        let correct = current_joltages
            .iter()
            .zip(self.joltage_requirements.iter())
            .all(|(current, wanted)| current == wanted);

        if correct {
            return Some(cost);
        }

        let this_recursion = current_joltages.to_vec();
        available_wiring_vecs
            .iter()
            .enumerate()
            .filter_map(|(i, wiring)| {
                assert!(current_joltages.len() == this_recursion.len());
                current_joltages.copy_from_slice(&this_recursion);
                let mut over = false;

                for joltage_i in wiring {
                    let joltage = current_joltages.get_mut(*joltage_i as usize).unwrap();
                    *joltage += 1;
                    over |= *joltage > self.joltage_requirements[*joltage_i as usize];
                }
                if over {
                    None
                } else {
                    self.joltages_recursive(&available_wiring_vecs[i..], current_joltages, cost + 1)
                }
            })
            .min()
    }

    fn least_squares(buttons_matrix: DMatrix<f32>, joltages: DVector<f32>) -> Option<DVector<f32>> {
        let m = buttons_matrix.nrows();
        let n = buttons_matrix.ncols();
        assert_eq!(joltages.len(), m);

        if m > n {
            return None;
        }

        let mut combinations = Vec::new();
        fn rec(start: usize, k: usize, cur: &mut Vec<usize>, n: usize, out: &mut Vec<Vec<usize>>) {
            if k == 0 {
                out.push(cur.clone());
                return;
            }
            for i in start..=(n - k) {
                cur.push(i);
                rec(i + 1, k - 1, cur, n, out);
                cur.pop();
            }
        }
        let mut cur = Vec::new();
        rec(0, m, &mut cur, n, &mut combinations);

        for cols in combinations {
            let mut sub = DMatrix::<f32>::zeros(m, m);
            for (j_sub, &j) in cols.iter().enumerate() {
                for i in 0..m {
                    sub[(i, j_sub)] = buttons_matrix[(i, j)];
                }
            }
            let lu = LU::new(sub.clone());
            if !lu.is_invertible() {
                continue;
            }

            if let Some(x_sub) = lu.solve(&joltages) {
                let mut x = DVector::<f32>::zeros(n);
                for (j_sub, &j) in cols.iter().enumerate() {
                    x[j] = x_sub[j_sub];
                }
                return Some(x);
            }
        }

        None
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

    let example_1 = solution_1(&example_1);
    dbg!(&example_1);
    assert_eq!(example_1, 7);

    let start = Instant::now();
    let solution_1 = solution_1(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 1: {elapsed}µs");
    dbg!(solution_1);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_2 = solution_2(&example_2);
    dbg!(&example_2);
    assert_eq!(example_2, 33);

    let start = Instant::now();
    let solution_2 = solution_2(_my_input);
    let elapsed = start.elapsed().as_micros();
    println!("Time to solve problem 2: {elapsed}µs");
    dbg!(solution_2);
}

fn solution_1(input: &[String]) -> u64 {
    let requirements = parse_input(input);
    requirements
        .into_iter()
        .map(|req| req.solve_requirements() as u64)
        .sum()
}

fn solution_2(input: &[String]) -> u64 {
    let requirements = parse_input(input);
    requirements
        .into_iter()
        .map(|req| dbg!(req.solve_joltages() as u64))
        .sum()
}

fn parse_input(input: &[String]) -> Vec<MachineRequirements> {
    input.iter().map(MachineRequirements::new).collect()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
