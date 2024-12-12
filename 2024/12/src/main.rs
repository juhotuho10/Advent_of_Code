/*
part 1:
we have square garden plot with different regions of different plants growing in it
we need to build a fence around each of the planting regions
and we need to know the area of the plants inside and the perimiter of the fence for the plants

we have to get each area * it's perimiter and return this sum for the whole garden plot

part 2:

instead of counting individual side pieces of the fence the whole straing fence now counts as a single piece of fence

*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn up(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn right(&self) -> Self {
        Coord {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn down(&self) -> Self {
        Coord {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Coord {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Plot {
    plant: char,
    flooded: bool,
    cheched: bool,
}

struct Garden {
    garden: HashMap<Coord, Plot>,
}

impl Garden {
    fn flood_plot(&mut self, coord: Coord, prev_plant: char) -> u32 {
        let plot = match self.garden.get_mut(&coord) {
            Some(plot) => plot,
            None => return 1,
        };

        if plot.plant != prev_plant {
            return 1;
        }

        if plot.flooded {
            return 0;
        }

        plot.flooded = true;

        let plot = *plot;

        let up_count = self.flood_plot(coord.up(), plot.plant);
        let right_count = self.flood_plot(coord.right(), plot.plant);
        let down_count = self.flood_plot(coord.down(), plot.plant);
        let left_count = self.flood_plot(coord.left(), plot.plant);

        up_count + right_count + down_count + left_count
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

    let example_sum = get_garden_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 1930);

    let my_sum = get_garden_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_garden_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 1206);

    let my_sum = get_garden_sum_2(_my_input);
    dbg!(my_sum);
}

fn get_garden_sum_1(input: &[String]) -> u32 {
    let mut garden = parse_input(input);

    let mut plant_fence_sum = 0;

    loop {
        let coord_plot = garden
            .garden
            .iter()
            .find(|(_, plot)| !plot.cheched)
            .map(|(coord, plot)| (*coord, *plot));

        match coord_plot {
            Some((coord, plot)) => {
                let fence_count = garden.flood_plot(coord, plot.plant);

                let plant_count = garden
                    .garden
                    .values()
                    .filter(|plot| plot.flooded && !plot.cheched)
                    .count();

                plant_fence_sum += fence_count * plant_count as u32;

                for (_, plot) in garden.garden.iter_mut() {
                    if plot.flooded {
                        plot.cheched = true;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    plant_fence_sum
}

fn get_garden_sum_2(input: &[String]) -> u32 {
    let mut garden = parse_input(input);

    let mut plant_fence_sum = 0;

    loop {
        let coord_plot = garden
            .garden
            .iter()
            .find(|(_, plot)| !plot.cheched)
            .map(|(coord, plot)| (*coord, *plot));

        match coord_plot {
            Some((coord, plot)) => {
                garden.flood_plot(coord, plot.plant);

                let flooded_coords: Vec<Coord> = garden
                    .garden
                    .iter()
                    .filter(|(_, plot)| plot.flooded && !plot.cheched)
                    .map(|(coord, _)| *coord)
                    .collect();

                let plant_count = flooded_coords.len();

                let mut total_plot_corner_count: u32 = 0;

                for flood_coord in &flooded_coords {
                    let mut surrounding = [[false; 3]; 3];

                    for (surroundind_y, y_diff) in (-1..=1).enumerate() {
                        for (surroundind_x, x_diff) in (-1..=1).enumerate() {
                            let mut checking_coord = *flood_coord;
                            checking_coord.x += x_diff;
                            checking_coord.y += y_diff;

                            let is_surrounding = flooded_coords.contains(&checking_coord);

                            surrounding[surroundind_y][surroundind_x] = is_surrounding;
                        }
                    }

                    total_plot_corner_count += check_corner_count(surrounding) as u32
                }

                plant_fence_sum += total_plot_corner_count * plant_count as u32;

                for (_, plot) in garden.garden.iter_mut() {
                    if plot.flooded {
                        plot.cheched = true;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    plant_fence_sum
}

fn check_corner_count(mut array: [[bool; 3]; 3]) -> u8 {
    let mut total = 0;

    for _ in 0..4 {
        if check_inside_corner(&array) {
            total += 1;
        }

        if check_outside_corner(&array) {
            total += 1;
        }

        array = rotate(array);
    }

    total
}
fn check_inside_corner(array: &[[bool; 3]; 3]) -> bool {
    if array[1][0] && array[0][1] && !array[0][0] {
        return true;
    }

    false
}

fn check_outside_corner(array: &[[bool; 3]; 3]) -> bool {
    if !array[1][0] && !array[0][1] {
        return true;
    }

    false
}

fn rotate(array: [[bool; 3]; 3]) -> [[bool; 3]; 3] {
    let mut rotated = [[false; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            rotated[j][2 - i] = array[i][j];
        }
    }

    rotated
}

fn parse_input(input: &[String]) -> Garden {
    let mut garden: HashMap<Coord, Plot> = HashMap::new();
    for (y, y_line) in input.iter().enumerate() {
        for (x, x_char) in y_line.char_indices() {
            let current_coord = Coord {
                x: x as i32,
                y: y as i32,
            };
            let new_plot = Plot {
                plant: x_char,
                flooded: false,
                cheched: false,
            };

            garden.insert(current_coord, new_plot);
        }
    }

    Garden { garden }
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
