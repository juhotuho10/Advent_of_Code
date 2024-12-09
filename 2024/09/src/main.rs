/*
part 1:
we have a disk with numbers on it, the numbers on the disk represent the file size and free spaces on the disk alternating with eachother
a number like 12345 would be 1: file, 2:free, 3: file, 4: free, 5: file

each file has an ID starting from 0 and counting up

we have to move files so that all the free space is moved to the end of the file.
this happends one datablock at a time so that the right most data gets sent left first

then we have to get the checksum from the re ordered disk so that we sum the num * position index and return that

part 2:

we have to move the files as whole from right to left, and we only get to check the files once so if they cannot be moved, we don't move them

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct DiskFile {
    num: Option<u64>,
    size: usize,
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = disk_checksum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 1928);

    let my_sum = disk_checksum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = disk_checksum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 2858);

    let my_sum = disk_checksum_2(_my_input);
    dbg!(my_sum);
}

fn disk_checksum_1(input: &[String]) -> u64 {
    let disk = &input[0];

    let mut expanded_disk: Vec<Option<u64>> = vec![];

    let mut current_id = 0;

    for (i, num_c) in disk.chars().enumerate() {
        let num = num_c.to_digit(10).unwrap();
        let push_num: Option<u64> = if i % 2 == 0 { Some(current_id) } else { None };

        if num == 0 {
            continue;
        }

        for _ in 0..num {
            expanded_disk.push(push_num);
        }

        if push_num.is_some() {
            current_id += 1;
        }
    }

    let mut left_pointer = 0;
    let mut right_pointer = expanded_disk.len() - 1;

    while left_pointer < right_pointer {
        while expanded_disk[left_pointer].is_some() {
            left_pointer += 1;
        }

        while expanded_disk[right_pointer].is_none() {
            right_pointer -= 1;
        }

        expanded_disk.swap(left_pointer, right_pointer);
    }

    expanded_disk
        .iter()
        .filter(|num| num.is_some())
        .enumerate()
        .map(|(i, num)| i as u64 * num.unwrap())
        .sum()
}

#[allow(clippy::comparison_chain)]
fn disk_checksum_2(input: &[String]) -> u64 {
    let disk = &input[0];

    let mut expanded_disk: Vec<DiskFile> = vec![];

    let mut current_id = 0;

    for (i, num_c) in disk.chars().enumerate() {
        let num = num_c.to_digit(10).unwrap();
        if num == 0 {
            continue;
        }
        let push_num: Option<u64> = if i % 2 == 0 { Some(current_id) } else { None };

        let new_diskfile = DiskFile {
            num: push_num,
            size: num as usize,
        };

        expanded_disk.push(new_diskfile);

        if push_num.is_some() {
            current_id += 1;
        }
    }

    let mut right_diskfile_index = expanded_disk.len() - 1;

    while right_diskfile_index > 0 {
        let right_diskfile = expanded_disk[right_diskfile_index];
        if right_diskfile.num.is_none() {
            right_diskfile_index -= 1;
            continue;
        }

        let mut left_diskfile_index = 0;
        let mut inserted = false;

        while left_diskfile_index < right_diskfile_index {
            let left_diskfile = expanded_disk[left_diskfile_index];
            if left_diskfile.num.is_some() {
                left_diskfile_index += 1;
                continue;
            }

            if left_diskfile.size < right_diskfile.size {
                left_diskfile_index += 1;
                continue;
            } else if left_diskfile.size == right_diskfile.size {
                expanded_disk.swap(left_diskfile_index, right_diskfile_index);
                break;
            } else {
                // left > right
                let right_file = expanded_disk.get_mut(right_diskfile_index).unwrap();
                right_file.num = None;

                let left_file = expanded_disk.get_mut(left_diskfile_index).unwrap();
                left_file.size -= right_diskfile.size;

                expanded_disk.insert(left_diskfile_index, right_diskfile);
                inserted = true;
                break;
            }
        }

        if !inserted {
            right_diskfile_index -= 1;
        }
    }

    let mut file_bytes: Vec<Option<u64>> = vec![];

    for file in expanded_disk {
        for _ in 0..file.size {
            file_bytes.push(file.num);
        }
    }
    file_bytes
        .iter()
        .enumerate()
        .map(|(i, num)| i as u64 * num.unwrap_or(0))
        .sum()
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
