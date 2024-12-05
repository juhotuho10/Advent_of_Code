/*
part 1:
we have page number ordering rules for example:
24 | 32 means that 24 always comes before 32, but not necessarily right before 32
and we have updates that are sets of numbers:
75,47,61,53,29
and we have to see if the sets of numbers are in correct orders according to the update rules

and then we add up the middle numbers of the correct updates and return the sum as an answer
part 2:

we instead have to take the incorrectly ordered update sets and order them correctly, then take the middle number from those and add those up
*/

use indexmap::IndexSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Order {
    rules: Vec<OrderingRule>,
}
impl Order {
    fn create_order_ver(rules: Vec<OrderingRule>) -> IndexSet<u32> {
        let mut temp_rules = rules.clone();
        let mut current_orders: IndexSet<u32> = IndexSet::new();

        let mut stop_counter = 0;
        while !temp_rules.is_empty() {
            let everything_after: IndexSet<u32> =
                temp_rules.iter().map(|rule| rule.after).collect();
            let exclusively_before: Vec<u32> = temp_rules
                .iter()
                .map(|rule| rule.before)
                .filter(|before_nums| !everything_after.contains(before_nums))
                .collect();

            if stop_counter == 40 {
                for i in temp_rules {
                    println!("{}, {}", i.before, i.after);
                }
                panic!();
            }
            stop_counter += 1;
            temp_rules.retain(|rule| !exclusively_before.contains(&rule.before));
            current_orders.extend(exclusively_before);
        }

        for rule in &rules {
            current_orders.insert(rule.after);
        }

        current_orders
    }

    fn update_batch_order(&self, update: &UpdateBatch) -> IndexSet<u32> {
        let relevant_orders: Vec<OrderingRule> = self
            .rules
            .clone()
            .into_iter()
            .filter(|rule| {
                update.batch.contains(&rule.before) && update.batch.contains(&rule.after)
            })
            .collect();

        Order::create_order_ver(relevant_orders)
    }
}

#[derive(Debug, Clone, Copy)]
struct OrderingRule {
    before: u32,
    after: u32,
}

impl OrderingRule {
    fn from_string(input: String) -> Self {
        let (a, b) = input.split_once("|").unwrap();
        OrderingRule {
            before: a.parse().unwrap(),
            after: b.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct UpdateBatch {
    batch: Vec<u32>,
}

impl UpdateBatch {
    fn from_string(input: String) -> Self {
        let nums: Vec<u32> = input
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        UpdateBatch { batch: nums }
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

    let example_sum = get_update_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 143);

    let my_sum = get_update_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = get_update_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 123);

    let my_sum = get_update_sum_2(_my_input);
    dbg!(my_sum);
}

fn get_update_sum_1(input: &[String]) -> u32 {
    let (ordering_rules, updates) = parse_input(input);
    let order_rules = Order {
        rules: ordering_rules,
    };

    let mut good_upates = vec![];

    for update in updates {
        let order_vec = order_rules.update_batch_order(&update);

        let order_index_vec: Vec<usize> = update_batch_order_index(&update, &order_vec);

        if order_index_vec.is_sorted() {
            good_upates.push(update.batch);
        }
    }

    let mut middle_sum = 0;
    for update in good_upates {
        middle_sum += update[update.len() / 2];
    }

    middle_sum
}

fn get_update_sum_2(input: &[String]) -> u32 {
    let (ordering_rules, updates) = parse_input(input);
    let order_rules = Order {
        rules: ordering_rules,
    };

    let mut bad_updates = vec![];

    for update in updates {
        let mut order_vec = order_rules.update_batch_order(&update);

        let order_index_vec: Vec<usize> = update_batch_order_index(&update, &order_vec);

        if !order_index_vec.is_sorted() {
            order_vec.retain(|num| update.batch.contains(num));
            bad_updates.push(order_vec);
        }
    }

    let mut middle_sum = 0;
    for update in bad_updates {
        middle_sum += update[update.len() / 2];
    }

    middle_sum
}

fn update_batch_order_index(update: &UpdateBatch, order_vec: &IndexSet<u32>) -> Vec<usize> {
    update
        .batch
        .iter()
        .map(|batch_num| {
            order_vec
                .iter()
                .position(|order_num: &u32| order_num == batch_num)
                .unwrap()
        })
        .collect()
}

fn parse_input(input: &[String]) -> (Vec<OrderingRule>, Vec<UpdateBatch>) {
    let mut ordering_strings: Vec<String> = vec![];
    let mut update_strings: Vec<String> = vec![];
    let mut push_order = true;
    for s in input {
        if s.is_empty() {
            push_order = false;
            continue;
        }

        if push_order {
            ordering_strings.push(s.clone());
        } else {
            update_strings.push(s.clone());
        }
    }

    let orederings = ordering_strings
        .into_iter()
        .map(OrderingRule::from_string)
        .collect();

    let updates = update_strings
        .into_iter()
        .map(UpdateBatch::from_string)
        .collect();

    (orederings, updates)
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
