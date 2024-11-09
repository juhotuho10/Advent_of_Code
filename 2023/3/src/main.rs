/*
input is a matrix of characters
if character is number and the number is connected to other numbers on the same row, they all belong to the same number
part1:
    if any numbers has any symbol other than "." next to it or even diagonal to that number, the number belongs to a part
    the answer is the sum of all part numbers

part2:
    if 2 numbers are connected by a '*' symbol, they belong to a gear ratio
    all gear ratios are multiplied and the sum of all multiplied gear ratios is returned
*/
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);
    let example_sum = get_part_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 4361);

    let my_sum = get_part_sum_1(_my_input);
    dbg!(&my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);
    let example_sum = get_gear_ratio_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 467835);

    let my_sum = get_gear_ratio_sum_2(_my_input);
    dbg!(&my_sum);
}

fn get_part_sum_1(input: &[String]) -> u32 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Coord {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct PartChars {
        symbol: char,
        next_to_symbol: bool,
    }

    impl PartChars {
        fn new(symbol: char) -> Self {
            PartChars {
                symbol,
                next_to_symbol: false,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct WholeParts {
        num_string: String,
        next_to_symbol: bool,
    }

    let mut char_matrix: HashMap<Coord, PartChars> = HashMap::new();

    for (y, y_char_list) in input.iter().enumerate() {
        for (x, x_char) in y_char_list.chars().enumerate() {
            char_matrix.insert(
                Coord {
                    x: x as i32,
                    y: y as i32,
                },
                PartChars::new(x_char),
            );
        }
    }

    let surrounding = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (y, y_char_list) in input.iter().enumerate() {
        for (x, x_char) in y_char_list.chars().enumerate() {
            if x_char.is_numeric() {
                for (x_offset, y_offset) in surrounding {
                    let surrounding_x = x as i32 + x_offset;
                    let surrounding_y = y as i32 + y_offset;

                    let surrounding_part = char_matrix.get(&Coord {
                        x: surrounding_x,
                        y: surrounding_y,
                    });

                    if let Some(surrounding_part) = surrounding_part {
                        if !surrounding_part.symbol.is_numeric() && surrounding_part.symbol != '.' {
                            let current_coord = Coord {
                                x: x as i32,
                                y: y as i32,
                            };
                            char_matrix.get_mut(&current_coord).unwrap().next_to_symbol = true;
                        }
                    }
                }
            }
        }
    }

    let mut parts_vec: Vec<WholeParts> = vec![];

    for (y, y_char_list) in input.iter().enumerate() {
        let mut current_part_str: Option<String> = None;
        let mut next_to_part = false;
        for (x, x_char) in y_char_list.chars().enumerate() {
            let current_coord = Coord {
                x: x as i32,
                y: y as i32,
            };

            let current_part = char_matrix[&current_coord];

            match current_part.symbol.is_numeric() {
                true => {
                    if current_part.next_to_symbol {
                        next_to_part = true;
                    };

                    match current_part_str {
                        Some(part_str) => current_part_str = Some(part_str + &x_char.to_string()),
                        None => current_part_str = Some(x_char.to_string()),
                    }
                }
                false => {
                    if let Some(part_str_) = current_part_str {
                        let new_part = WholeParts {
                            num_string: part_str_,
                            next_to_symbol: next_to_part,
                        };

                        parts_vec.push(new_part);
                        current_part_str = None;
                        next_to_part = false;
                    }
                }
            };

            let is_last_index = x == y_char_list.len() - 1;
            if is_last_index {
                if let Some(part_str_) = current_part_str {
                    let new_part = WholeParts {
                        num_string: part_str_,
                        next_to_symbol: next_to_part,
                    };

                    parts_vec.push(new_part);
                    current_part_str = None;
                    next_to_part = false;
                }
            }
        }
    }

    parts_vec
        .iter()
        .filter(|part| part.next_to_symbol)
        .map(|part| part.num_string.parse::<u32>().unwrap())
        .sum()
}

fn get_gear_ratio_sum_2(input: &[String]) -> u32 {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Coord {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct PartChars {
        symbol: char,
        symbol_location: Option<Coord>,
    }

    impl PartChars {
        fn new(symbol: char) -> Self {
            PartChars {
                symbol,
                symbol_location: None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct WholeParts {
        num_string: String,
        gear_coordinates: Option<Coord>,
    }

    let mut char_matrix: HashMap<Coord, PartChars> = HashMap::new();

    let mut gear_locations: HashSet<Coord> = HashSet::new();

    for (y, y_char_list) in input.iter().enumerate() {
        for (x, x_char) in y_char_list.chars().enumerate() {
            char_matrix.insert(
                Coord {
                    x: x as i32,
                    y: y as i32,
                },
                PartChars::new(x_char),
            );
        }
    }

    let surrounding = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (y, y_char_list) in input.iter().enumerate() {
        for (x, x_char) in y_char_list.chars().enumerate() {
            if x_char.is_numeric() {
                for (x_offset, y_offset) in surrounding {
                    let surrounding_x = x as i32 + x_offset;
                    let surrounding_y = y as i32 + y_offset;

                    let surrounding_coord = Coord {
                        x: surrounding_x,
                        y: surrounding_y,
                    };

                    let surrounding_part = char_matrix.get(&surrounding_coord);

                    if let Some(surrounding_part) = surrounding_part {
                        if !surrounding_part.symbol.is_numeric() && surrounding_part.symbol == '*' {
                            gear_locations.insert(surrounding_coord);
                            let current_coord = Coord {
                                x: x as i32,
                                y: y as i32,
                            };
                            char_matrix.get_mut(&current_coord).unwrap().symbol_location =
                                Some(surrounding_coord);
                        }
                    }
                }
            }
        }
    }

    let mut parts_vec: Vec<WholeParts> = vec![];

    for (y, y_char_list) in input.iter().enumerate() {
        let mut current_part_str: Option<String> = None;
        let mut gear_coord: Option<Coord> = None;
        for (x, x_char) in y_char_list.chars().enumerate() {
            let current_coord = Coord {
                x: x as i32,
                y: y as i32,
            };

            let current_part = char_matrix[&current_coord];

            match current_part.symbol.is_numeric() {
                true => {
                    if let Some(location) = current_part.symbol_location {
                        gear_coord = Some(location);
                    };

                    match current_part_str {
                        Some(part_str) => current_part_str = Some(part_str + &x_char.to_string()),
                        None => current_part_str = Some(x_char.to_string()),
                    }
                }
                false => {
                    if let Some(part_str_) = current_part_str {
                        let new_part = WholeParts {
                            num_string: part_str_,
                            gear_coordinates: gear_coord,
                        };

                        parts_vec.push(new_part);

                        current_part_str = None;
                        gear_coord = None;
                    }
                }
            };

            let is_last_index = x == y_char_list.len() - 1;
            if is_last_index {
                if let Some(part_str_) = current_part_str {
                    let new_part = WholeParts {
                        num_string: part_str_,
                        gear_coordinates: gear_coord,
                    };

                    parts_vec.push(new_part);
                    current_part_str = None;
                    gear_coord = None;
                }
            }
        }
    }

    let valid_parts: Vec<&WholeParts> = parts_vec
        .iter()
        .filter(|&part| part.gear_coordinates.is_some())
        .collect();

    let mut combined_ratios: Vec<u32> = vec![];
    for location in gear_locations {
        let gear_parts: Vec<&&WholeParts> = valid_parts
            .iter()
            .filter(|part| part.gear_coordinates.unwrap() == location)
            .collect();

        if gear_parts.len() != 2 {
            continue;
        }

        let gear_1 = gear_parts[0].num_string.parse::<u32>().unwrap();
        let gear_2 = gear_parts[1].num_string.parse::<u32>().unwrap();

        combined_ratios.push(gear_1 * gear_2);
    }

    combined_ratios.iter().sum()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
