/*
part 1:
we have a robot with a number pad controlled by another robot with a keypad controlled by another robot with a keypad controlled by another robot with a keypad.
input -> keypad -> keypad -> keypad -> numberpad

we need to return the min number of keypresses needed for the input
part 2:

we now have 26 keypads before the number pad and we need to find the min amound of keypresses needed

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u32,
    y: u32,
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

    fn go_dir(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => self.up(),
            Dir::Left => self.left(),
            Dir::Down => self.down(),
            Dir::Right => self.right(),
        }
    }
}

#[derive(Debug)]
struct Numpad {
    numpad: HashMap<Coord, char>,
}

impl Numpad {
    fn new() -> Self {
        let mut numpad: HashMap<Coord, char> = HashMap::new();

        numpad.insert(Coord { x: 0, y: 0 }, '7');
        numpad.insert(Coord { x: 1, y: 0 }, '8');
        numpad.insert(Coord { x: 2, y: 0 }, '9');
        numpad.insert(Coord { x: 0, y: 1 }, '4');
        numpad.insert(Coord { x: 1, y: 1 }, '5');
        numpad.insert(Coord { x: 2, y: 1 }, '6');
        numpad.insert(Coord { x: 0, y: 2 }, '1');
        numpad.insert(Coord { x: 1, y: 2 }, '2');
        numpad.insert(Coord { x: 2, y: 2 }, '3');
        numpad.insert(Coord { x: 1, y: 3 }, '0');
        numpad.insert(Coord { x: 2, y: 3 }, 'A');

        Numpad { numpad }
    }
}

#[derive(Debug)]
struct Keypad {
    keypad: HashMap<Coord, char>,
}

impl Keypad {
    fn new() -> Self {
        let mut keypad: HashMap<Coord, char> = HashMap::new();

        keypad.insert(Coord { x: 1, y: 0 }, '^');
        keypad.insert(Coord { x: 2, y: 0 }, 'A');
        keypad.insert(Coord { x: 0, y: 1 }, '<');
        keypad.insert(Coord { x: 1, y: 1 }, 'v');
        keypad.insert(Coord { x: 2, y: 1 }, '>');

        Keypad { keypad }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct NodeSearcher {
    accumulated_cost: u32,
    current_coord: Coord,
    facing_dir: Dir,
    dirs_taken: Vec<char>,
}

impl Ord for NodeSearcher {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.accumulated_cost.cmp(&other.accumulated_cost)
    }
}

impl PartialOrd for NodeSearcher {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn common_djikstra(
    djikstra_cache: &mut HashMap<(char, char), Vec<Vec<char>>>,
    character_pad: &HashMap<Coord, char>,
    start_char: &char,
    end_char: &char,
) -> Vec<Vec<char>> {
    // return vec of chars is the instructions to get to the end, for example >>^^

    if let Some(cached_result) = djikstra_cache.get(&(*start_char, *end_char)) {
        return cached_result.clone();
    }

    let mut cost_heap: BinaryHeap<Reverse<NodeSearcher>> = BinaryHeap::new();

    let start_coord = character_pad
        .iter()
        .find(|(_, char)| *char == start_char)
        .map(|(coord, _)| *coord)
        .unwrap();

    for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
        let start_searcher = NodeSearcher {
            accumulated_cost: 0,
            current_coord: start_coord,
            facing_dir: dir,
            dirs_taken: vec![],
        };

        cost_heap.push(Reverse(start_searcher));
    }

    let mut min_cost = None;

    let mut possible_paths = vec![];

    while let Some(rev_searcher) = cost_heap.pop() {
        let searcher = rev_searcher.0;

        if let Some(&curr_char) = character_pad.get(&searcher.current_coord) {
            if *end_char == curr_char {
                if let Some(cost) = min_cost {
                    if cost < searcher.accumulated_cost {
                        break;
                    }
                } else {
                    min_cost = Some(searcher.accumulated_cost);
                }

                possible_paths.push(searcher.dirs_taken);
                continue;
            }
        }

        for new_dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
            let new_coord = searcher.current_coord.go_dir(new_dir);
            if character_pad.contains_key(&new_coord) {
                let mut copy_searcher = searcher.clone();

                copy_searcher.current_coord = new_coord;

                let new_cost = if new_dir == searcher.facing_dir {
                    searcher.accumulated_cost + 1
                } else {
                    searcher.accumulated_cost + 2
                };

                let new_dir_char = match new_dir {
                    Dir::Up => '^',
                    Dir::Right => '>',
                    Dir::Down => 'v',
                    Dir::Left => '<',
                };
                copy_searcher.dirs_taken.push(new_dir_char);

                copy_searcher.accumulated_cost = new_cost;
                copy_searcher.facing_dir = new_dir;
                cost_heap.push(Reverse(copy_searcher));
            }
        }
    }

    djikstra_cache.insert((*start_char, *end_char), possible_paths.clone());

    possible_paths
}

fn recursive_keypad_pusher(
    use_numpad: bool,
    numpad: &HashMap<Coord, char>,
    keypad: &HashMap<Coord, char>,
    input: Vec<char>,
    robots: u16,
    djikstra_cache: &mut HashMap<(char, char), Vec<Vec<char>>>,
    input_cache: &mut HashMap<(Vec<char>, u16), u64>,
) -> u64 {
    if robots == 0 {
        return input.len() as u64 - 1;
    }

    if let Some(&cached_result) = input_cache.get(&(input.clone(), robots)) {
        return cached_result;
    }

    let mut total_presses = 0;

    for key_pair in input.windows(2) {
        let new_directions = if use_numpad {
            common_djikstra(djikstra_cache, numpad, &key_pair[0], &key_pair[1])
        } else {
            common_djikstra(djikstra_cache, keypad, &key_pair[0], &key_pair[1])
        };

        let mut min_presses = u64::MAX;

        for new_dirs in new_directions {
            let mut press_sequence = vec!['A'];
            press_sequence.extend(new_dirs);
            press_sequence.push('A');

            let total_key_persses = recursive_keypad_pusher(
                false,
                numpad,
                keypad,
                press_sequence,
                robots - 1,
                djikstra_cache,
                input_cache,
            );

            min_presses = min_presses.min(total_key_persses);
        }

        total_presses += min_presses;
    }

    input_cache.insert((input, robots), total_presses);
    total_presses
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = min_keypresses(&example_1, 3);
    dbg!(&example_sum);
    assert_eq!(example_sum, 126384);

    let my_sum = min_keypresses(_my_input, 3);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let my_sum = min_keypresses(_my_input, 26);
    dbg!(my_sum);
}

fn min_keypresses(input: &[String], robot_count: u16) -> u64 {
    let mut total = 0;

    let mut djikstra_cache: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut input_cache: HashMap<(Vec<char>, u16), u64> = HashMap::new();

    for keypad in input {
        let keypad_num: u64 = keypad.chars().take(3).collect::<String>().parse().unwrap();

        let mut input_chars: Vec<char> = keypad.chars().collect();
        input_chars.insert(0, 'A');

        let total_presses = recursive_keypad_pusher(
            true,
            &Numpad::new().numpad,
            &Keypad::new().keypad,
            input_chars,
            robot_count,
            &mut djikstra_cache,
            &mut input_cache,
        );

        total += total_presses * keypad_num;
    }

    total
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
