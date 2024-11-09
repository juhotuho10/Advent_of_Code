/*
cubes that can be red, green or blue
n number of games from 1 to n
each game we have m number of rounds, where we pull cubes out of the bag
part 1:
    - the cubers are put back into the bag so we check max number of cubes each round in the game for each color
    - figure out if the game is possible with the set number of cubes
    - get all the possible game nums
    - sum up all the possible game numbers to get the correct answer

part 2:
    - the cubers are put back into the bag so we check max number of cubes each round in the game for each color
    - figure out if the lest number of each color that the game is possible with
    - multiply the cubes together for each game
    - return the sum

*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Game {
    number: u32,
    cubes: HashMap<Cubes, u32>,
}

impl Game {
    fn new(game_num: u32) -> Self {
        let mut cubes = HashMap::new();

        cubes.insert(Cubes::Red, 0);
        cubes.insert(Cubes::Green, 0);
        cubes.insert(Cubes::Blue, 0);

        Game {
            number: game_num,
            cubes,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Cubes {
    Red,
    Green,
    Blue,
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = valid_games_sum_1(&example_1);

    dbg!(&example_sum);

    assert_eq!(example_sum, 8);

    let my_sum = valid_games_sum_1(_my_input);

    dbg!(&my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = multiplied_cube_sum(&example_2);

    dbg!(&example_sum);

    assert_eq!(example_sum, 2286);

    let my_sum = multiplied_cube_sum(_my_input);

    dbg!(&my_sum);
}

fn parse_games(input: &[String]) -> Vec<Game> {
    let mut all_games: Vec<Game> = vec![];
    for whole_game in input {
        let (game, rounds_str) = whole_game.split_once(":").unwrap();
        let (_, game_num_str) = game.split_once(" ").unwrap();
        let game_num = game_num_str.to_owned().parse::<u32>().unwrap();
        let rounds: Vec<&str> = rounds_str.split(";").collect();

        let mut curent_game = Game::new(game_num);

        for round in rounds {
            let cubes: Vec<&str> = round.split(",").collect();
            for cube_str in cubes {
                let (cube_num_str, cube_color) = cube_str.trim().split_once(" ").unwrap();

                let cube_num = cube_num_str.to_owned().parse::<u32>().unwrap();
                let cube_type = match cube_color {
                    "red" => Cubes::Red,
                    "green" => Cubes::Green,
                    "blue" => Cubes::Blue,
                    _ => unreachable!(),
                };

                if curent_game.cubes[&cube_type] < cube_num {
                    curent_game.cubes.insert(cube_type, cube_num);
                }
            }
        }

        all_games.push(curent_game);
    }

    all_games
}

fn valid_games_sum_1(input: &[String]) -> u32 {
    let mut max_possible_cubes = HashMap::new();
    max_possible_cubes.insert(Cubes::Red, 12);
    max_possible_cubes.insert(Cubes::Green, 13);
    max_possible_cubes.insert(Cubes::Blue, 14);

    let all_games = parse_games(input);

    let possible_games: Vec<&Game> = all_games
        .iter()
        .filter(|game| {
            let game_cubes = &game.cubes;
            game_cubes[&Cubes::Red] <= max_possible_cubes[&Cubes::Red]
                && game_cubes[&Cubes::Green] <= max_possible_cubes[&Cubes::Green]
                && game_cubes[&Cubes::Blue] <= max_possible_cubes[&Cubes::Blue]
        })
        .collect();

    possible_games.iter().map(|game| game.number).sum()
}

fn multiplied_cube_sum(input: &[String]) -> u32 {
    let all_games = parse_games(input);

    all_games
        .iter()
        .map(|game| {
            let game_cubes = &game.cubes;
            game_cubes[&Cubes::Red] * game_cubes[&Cubes::Green] * game_cubes[&Cubes::Blue]
        })
        .sum()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("wont fail");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
