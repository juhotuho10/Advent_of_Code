/*
part 1:
we have collections of junction boxes to connect in a way that reaches every single junction box
the boxes are given in x,y,z coordinates
we have to connect boxes that are as close to each other as possible
after connecting together N closest pairs, we have a collection of circuits
now we multiply the sizes of the 3 largerst circuits together to get the answer
part 2:
instead of connecting N largest, connect all into a large circuit
we take the connection that is last needed for the single large circuit to be formed and we take those boxes X coordinates and multiply them together
*/

use ahash::AHashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type BoxI = u16;

struct BoxCoords {
    x: Vec<u32>,
    y: Vec<u32>,
    z: Vec<u32>,
}

impl BoxCoords {
    fn new(input: &[String]) -> Self {
        let mut x = Vec::new();
        let mut y = Vec::new();
        let mut z = Vec::new();

        for string in input {
            let split: Vec<&str> = string.splitn(3, ",").collect();
            assert!(split.len() == 3);
            x.push(split[0].parse().unwrap());
            y.push(split[1].parse().unwrap());
            z.push(split[2].parse().unwrap());
        }

        BoxCoords { x, y, z }
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

    let example_sum = solution_1(&example_1, 10);
    dbg!(&example_sum);
    assert_eq!(example_sum, 40);

    let start = Instant::now();
    let my_sum = solution_1(_my_input, 1000);
    let elapsed = start.elapsed().as_micros();
    println!("part 1 time elapsed: {elapsed} micros");
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = solution_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 25272);

    let my_sum = solution_2(_my_input);
    dbg!(my_sum);
}

fn solution_1(input: &[String], n_connections: usize) -> u64 {
    let boxes = parse_input(input);

    let n_boxes = boxes.x.len();
    let distance_capacity = (n_boxes * (n_boxes - 1)) / 2;
    // get all distances between chains
    let mut box_distance: Vec<(BoxI, BoxI, u32)> = Vec::with_capacity(distance_capacity + 1); // + 1 to avoid re-allocation even with 1 off   
    for start_i in 0..(n_boxes - 1) {
        let start_x = boxes.x[start_i];
        let start_y = boxes.y[start_i];
        let start_z = boxes.z[start_i];

        box_distance.extend(((start_i + 1)..n_boxes).map(|end_i| {
            let x_dist = (start_x.abs_diff(boxes.x[end_i]) as u64).pow(2);
            let y_dist = (start_y.abs_diff(boxes.y[end_i]) as u64).pow(2);
            let z_dist = (start_z.abs_diff(boxes.z[end_i]) as u64).pow(2);
            (
                start_i as u16,
                end_i as u16,
                ((x_dist + y_dist + z_dist) as f64).sqrt() as u32,
            )
        }));
    }
    assert!(box_distance.len() == distance_capacity);

    box_distance.sort_unstable_by_key(|(_, _, dist)| *dist);

    let mut connected_chains: AHashSet<(BoxI, BoxI)> = AHashSet::with_capacity(512);
    let mut chain_lens: Vec<u16> = vec![0; n_boxes];
    let mut part_of_chain: Vec<Option<BoxI>> = vec![None; n_boxes];

    // add chains together with a chain ID
    // if chains both have id, just mark the chains as connected
    for (box_i_1, box_i_2, _) in &box_distance[..n_connections] {
        assert_ne!(box_i_1, box_i_2);
        let [chain_1, chain_2] = part_of_chain
            .get_disjoint_mut([*box_i_1 as usize, *box_i_2 as usize])
            .unwrap();

        match (&chain_1, &chain_2) {
            (None, None) => {
                let chain_i = *box_i_1.min(box_i_2);
                chain_lens[chain_i as usize] = 2;

                *chain_1 = Some(chain_i);
                *chain_2 = Some(chain_i);
            }
            (None, Some(chain_i)) => {
                chain_lens[*chain_i as usize] += 1;
                *chain_1 = Some(*chain_i);
            }
            (Some(chain_i), None) => {
                chain_lens[*chain_i as usize] += 1;
                *chain_2 = Some(*chain_i)
            }
            (Some(chain_i_1), Some(chain_i_2)) => {
                if chain_i_1 != chain_i_2 {
                    let insert_key = if chain_i_1 < chain_i_2 {
                        (*chain_i_1, *chain_i_2)
                    } else {
                        (*chain_i_2, *chain_i_1)
                    };
                    connected_chains.insert(insert_key);
                }
            }
        }
    }

    // consolidate all the chains marked as connected together
    let mut total_connections: Vec<AHashSet<BoxI>> = connected_chains
        .into_iter()
        .map(|(item1, item2)| {
            let mut new_hashset = AHashSet::with_capacity(2);
            new_hashset.insert(item1);
            new_hashset.insert(item2);
            new_hashset
        })
        .collect();

    let mut consolidated = false;

    while !consolidated {
        let connections_len = total_connections.len();
        consolidated = true;

        for ids_1 in 0..(connections_len - 1) {
            for ids_2 in (ids_1 + 1)..connections_len {
                assert_ne!(ids_1, ids_2);
                let [chain_1, chain_2] =
                    total_connections.get_disjoint_mut([ids_1, ids_2]).unwrap();

                if !chain_1.is_disjoint(chain_2) {
                    consolidated = false;
                    chain_2.extend(chain_1.iter());
                    chain_1.clear();
                }
            }
        }
        total_connections.retain(|set| !set.is_empty());
    }

    // add the consolidated chain lens back to all chain lesn
    for chain_ids in total_connections {
        let total = chain_ids.iter().map(|id| chain_lens[*id as usize]).sum();
        for key in &chain_ids {
            chain_lens[*key as usize] = 0;
        }
        let min_key = chain_ids.iter().min().unwrap();
        chain_lens[*min_key as usize] = total;
    }

    // get the product of 3 longest chains
    chain_lens.sort();

    chain_lens
        .iter()
        .rev()
        .take(3)
        .map(|num| *num as u64)
        .product()
}

fn solution_2(input: &[String]) -> u64 {
    let boxes = parse_input(input);

    let n_boxes = boxes.x.len();
    let distance_capacity = (n_boxes * (n_boxes - 1)) / 2;
    // get all distances between chains
    let mut box_distance: Vec<(BoxI, BoxI, u32)> = Vec::with_capacity(distance_capacity + 1); // + 1 to avoid re-allocation even with 1 off   
    for start_i in 0..(n_boxes - 1) {
        let start_x = boxes.x[start_i];
        let start_y = boxes.y[start_i];
        let start_z = boxes.z[start_i];

        box_distance.extend(((start_i + 1)..n_boxes).map(|end_i| {
            let x_dist = (start_x.abs_diff(boxes.x[end_i]) as u64).pow(2);
            let y_dist = (start_y.abs_diff(boxes.y[end_i]) as u64).pow(2);
            let z_dist = (start_z.abs_diff(boxes.z[end_i]) as u64).pow(2);
            (
                start_i as u16,
                end_i as u16,
                ((x_dist + y_dist + z_dist) as f64).sqrt() as u32,
            )
        }));
    }
    assert!(box_distance.len() == distance_capacity);

    box_distance.sort_unstable_by_key(|(_, _, dist)| *dist);

    // add chains together with a chain ID
    // if chains both have id, just mark the chains as connected
    for n_connections in 0..box_distance.len() {
        let mut connected_chains: AHashSet<(BoxI, BoxI)> = AHashSet::with_capacity(512);
        let mut chain_lens: Vec<u16> = vec![0; n_boxes];
        let mut part_of_chain: Vec<Option<BoxI>> = vec![None; n_boxes];

        for (box_i_1, box_i_2, _) in &box_distance[..n_connections] {
            assert_ne!(box_i_1, box_i_2);
            let [chain_1, chain_2] = part_of_chain
                .get_disjoint_mut([*box_i_1 as usize, *box_i_2 as usize])
                .unwrap();

            match (&chain_1, &chain_2) {
                (None, None) => {
                    let chain_i = *box_i_1.min(box_i_2);
                    chain_lens[chain_i as usize] = 2;

                    *chain_1 = Some(chain_i);
                    *chain_2 = Some(chain_i);
                }
                (None, Some(chain_i)) => {
                    chain_lens[*chain_i as usize] += 1;
                    *chain_1 = Some(*chain_i);
                }
                (Some(chain_i), None) => {
                    chain_lens[*chain_i as usize] += 1;
                    *chain_2 = Some(*chain_i)
                }
                (Some(chain_i_1), Some(chain_i_2)) => {
                    if chain_i_1 != chain_i_2 {
                        let insert_key = if chain_i_1 < chain_i_2 {
                            (*chain_i_1, *chain_i_2)
                        } else {
                            (*chain_i_2, *chain_i_1)
                        };
                        connected_chains.insert(insert_key);
                    }
                }
            }
        }
        // consolidate all the chains marked as connected together
        let mut total_connections: Vec<AHashSet<BoxI>> = connected_chains
            .clone()
            .into_iter()
            .map(|(item1, item2)| {
                let mut new_hashset = AHashSet::with_capacity(2);
                new_hashset.insert(item1);
                new_hashset.insert(item2);
                new_hashset
            })
            .collect();

        let mut consolidated = false;

        let connections_len = total_connections.len();

        while !consolidated && connections_len > 0 {
            let connections_len = total_connections.len();
            consolidated = true;
            for ids_1 in 0..(connections_len - 1) {
                for ids_2 in (ids_1 + 1)..connections_len {
                    assert_ne!(ids_1, ids_2);
                    let [chain_1, chain_2] =
                        total_connections.get_disjoint_mut([ids_1, ids_2]).unwrap();

                    if !chain_1.is_disjoint(chain_2) {
                        consolidated = false;
                        chain_2.extend(chain_1.iter());
                        chain_1.clear();
                    }
                }
            }
            total_connections.retain(|set| !set.is_empty());
        }

        // add the consolidated chain lens back to all chain lesn
        for chain_ids in total_connections {
            let total = chain_ids.iter().map(|id| chain_lens[*id as usize]).sum();
            for key in &chain_ids {
                chain_lens[*key as usize] = 0;
            }
            let min_key = chain_ids.iter().min().unwrap();
            chain_lens[*min_key as usize] = total;
        }

        if *chain_lens.iter().max().unwrap() == n_boxes as u16 {
            let (box_i_1, box_i_2, _) = box_distance[n_connections - 1];

            let box_1_x = boxes.x[box_i_1 as usize];
            let box_2_x = boxes.x[box_i_2 as usize];
            return box_1_x as u64 * box_2_x as u64;
        }
    }

    unreachable!()
}

fn parse_input(input: &[String]) -> BoxCoords {
    BoxCoords::new(input)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
