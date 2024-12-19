/*
part 1:
we have towels and we arrange them to show designs
every towel has a pattern of colored stripes, that can be white w blue u, black b, red r or green g.

if we want a design rgrgr, we havet o get 2 towels with rg and one towel with r to make the design

we have to check which designs are possible with the given towels and return the count of possible designs

part 2:

we have to check how many different ways the designs can be made from available towels

*/

use fxhash::FxHashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    W,
    U,
    B,
    R,
    G,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Towel {
    colors: Vec<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Design {
    colors: Vec<Color>,
}

#[derive(Debug, Clone)]
struct TowelDesigns {
    towels: Vec<Towel>,
    wanted_designs: Vec<Design>,
}

impl TowelDesigns {
    fn from_string(input: &[String]) -> Self {
        let towels_strings: Vec<&str> = input[0].split(", ").collect();
        let design_string: &[String] = &input[2..];

        let mut towels = vec![];
        for towel in towels_strings {
            let mut towel_colors = vec![];
            for char in towel.chars() {
                let char_color = match char {
                    'w' => Color::W,
                    'u' => Color::U,
                    'b' => Color::B,
                    'r' => Color::R,
                    'g' => Color::G,
                    _ => unreachable!(),
                };

                towel_colors.push(char_color);
            }

            towels.push(Towel {
                colors: towel_colors,
            });
        }

        let mut designs = vec![];
        for towel in design_string {
            let mut design_colors = vec![];
            for char in towel.chars() {
                let char_color = match char {
                    'w' => Color::W,
                    'u' => Color::U,
                    'b' => Color::B,
                    'r' => Color::R,
                    'g' => Color::G,
                    _ => unreachable!(),
                };

                design_colors.push(char_color);
            }

            designs.push(Design {
                colors: design_colors,
            });
        }

        TowelDesigns {
            towels,
            wanted_designs: designs,
        }
    }

    fn recursive_design_finder(
        &self,
        design: Vec<Color>,
        memo_cache: &mut FxHashMap<Vec<Color>, u64>,
    ) -> u64 {
        if design.is_empty() {
            return 1;
        }

        if let Some(&cached_result) = memo_cache.get(&design) {
            return cached_result;
        }

        let mut possible = 0;

        for towel in &self.towels {
            if towel.colors.len() <= design.len() && towel.colors == design[..towel.colors.len()] {
                possible +=
                    self.recursive_design_finder(design[towel.colors.len()..].to_vec(), memo_cache)
            }
        }

        memo_cache.insert(design, possible);

        possible
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

    let example_num = num_possible_designs(&example_1);
    dbg!(&example_num);
    assert_eq!(example_num, 6);

    let start = Instant::now();
    let my_num = num_possible_designs(_my_input);
    dbg!(start.elapsed());
    dbg!(my_num);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = num_total_designs(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 16);

    let start = Instant::now();
    let my_sum = num_total_designs(_my_input);
    dbg!(start.elapsed());
    dbg!(my_sum);
}

fn num_possible_designs(input: &[String]) -> u64 {
    let towel_designs = TowelDesigns::from_string(input);
    let mut possible = 0;
    let mut memo_cache: FxHashMap<Vec<Color>, u64> = FxHashMap::default();
    for design in &towel_designs.wanted_designs {
        if towel_designs.recursive_design_finder(design.colors.to_vec(), &mut memo_cache) > 0 {
            possible += 1;
        }
    }

    possible
}

fn num_total_designs(input: &[String]) -> u64 {
    let towel_designs = TowelDesigns::from_string(input);
    let mut total = 0;
    let mut memo_cache: FxHashMap<Vec<Color>, u64> = FxHashMap::default();
    for design in towel_designs.clone().wanted_designs {
        total += towel_designs.recursive_design_finder(design.colors.to_vec(), &mut memo_cache)
    }

    total
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
