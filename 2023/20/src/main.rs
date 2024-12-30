/*
part 1:
we have modules communicating to eachother with low pulses or high pulses

different modules:
(%) flipflop:
    on or off, start with off
    ignores high pulse
    low pulse flips it
        was off -> high pulse & flip
        was on -> low pulse & flip

(&) conjunction:
    rememebers received pulses
    starts with remembering low pulses
        if all inputs are high, it sends low
        else send high pulse

broadcaster:
    receives a pulse
    sends the same pulse to all the destinations

part 2:

*/

use fxhash::FxHashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModType {
    Flipflop,
    Conjunction,
    Broadcaster,
    Button,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    mod_type: ModType,
    state: bool,
    send_queue: VecDeque<bool>,
    received: FxHashMap<String, bool>,
    send_targets: Vec<String>,
}

impl Module {
    fn new(module: ModType, targets: Vec<String>) -> Self {
        Module {
            mod_type: module,
            state: false,
            send_queue: VecDeque::new(),
            received: FxHashMap::default(),
            send_targets: targets,
        }
    }

    fn receive(&mut self, sender: String, high_pulse: bool) {
        match self.mod_type {
            ModType::Flipflop => {
                if !high_pulse {
                    self.send_queue.push_back(!self.state);
                    self.state = !self.state;
                }
            }
            ModType::Conjunction => {
                self.received.insert(sender, high_pulse);
                let all_high = self.received.values().all(|&high_pulse| high_pulse);
                self.send_queue.push_back(!all_high);
            }
            ModType::Broadcaster => self.send_queue.push_back(high_pulse),

            ModType::Button => unreachable!(),
        }
    }

    fn send(&mut self) -> Option<bool> {
        match self.mod_type {
            ModType::Flipflop | ModType::Broadcaster | ModType::Conjunction => {
                self.send_queue.pop_front()
            }
            ModType::Button => Some(false),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AllModules {
    modules: FxHashMap<String, Module>,
}
impl AllModules {
    fn from_string(input: &[String]) -> Self {
        let mut modules: FxHashMap<String, Module> = FxHashMap::default();
        modules.insert(
            "button".to_owned(),
            Module::new(ModType::Button, vec!["broadcaster".to_owned()]),
        );

        let mut senders = vec![];

        let mut all_receivers = vec![];

        for line in input {
            let (send, receivers_str) = line.split_once(" -> ").unwrap();
            let receivers: Vec<String> = receivers_str.split(", ").map(|s| s.to_owned()).collect();

            let (mod_type, send_name) = if send.starts_with("%") {
                (ModType::Flipflop, send.strip_prefix("%").unwrap())
            } else if send.starts_with("&") {
                (ModType::Conjunction, send.strip_prefix("&").unwrap())
            } else if send == "broadcaster" {
                (ModType::Broadcaster, send)
            } else {
                unreachable!()
            };

            senders.push(send_name);
            all_receivers.push(receivers.clone());

            let new_module = Module::new(mod_type, receivers);
            modules.insert(send_name.to_owned(), new_module);
        }

        //let conjunctions: Vec<&mut Module> = modules
        //    .values_mut()
        //    .filter(|m| m.mod_type == ModType::Conjunction)
        //    .collect();

        for (sender, receivers) in senders.iter().zip(all_receivers) {
            for rec in receivers {
                if let Some(module) = modules.get_mut(&rec) {
                    if module.mod_type == ModType::Conjunction {
                        module.received.insert(sender.to_string(), false);
                    }
                }
            }
        }

        AllModules { modules }
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

    let example_sum = pulse_total_sum_1(&example_1, 1000);
    dbg!(&example_sum);
    assert_eq!(example_sum, 11687500);

    let my_sum = pulse_total_sum_1(_my_input, 1000);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let my_sum = pulse_total_sum_2(_my_input);
    dbg!(my_sum);
}

fn pulse_total_sum_1(input: &[String], sent_pulses: u32) -> u32 {
    let mut all_modules = AllModules::from_string(input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..sent_pulses {
        let mut module_queue = VecDeque::from(vec!["button".to_owned()]);

        while let Some(module_str) = module_queue.pop_front() {
            let (send_sign, send_targets) = {
                let current_module = all_modules.modules.get_mut(&module_str).unwrap();
                if let Some(return_sng) = current_module.send() {
                    let targets = current_module.send_targets.clone();

                    if return_sng {
                        high_pulses += targets.len() as u32;
                    } else {
                        low_pulses += targets.len() as u32;
                    }

                    (return_sng, targets)
                } else {
                    continue;
                }
            };

            for target_str in send_targets {
                if let Some(target_mod) = all_modules.modules.get_mut(&target_str) {
                    target_mod.receive(module_str.clone(), send_sign);
                    module_queue.push_back(target_str);
                }
            }
        }
    }

    low_pulses * high_pulses
}

fn pulse_total_sum_2(input: &[String]) -> u64 {
    let mut all_modules = AllModules::from_string(input);

    // single module feeding the rx module
    let rx_feeder = all_modules
        .modules
        .iter()
        .find(|(_, val)| val.send_targets.contains(&"rx".to_string()))
        .map(|(key, _)| key)
        .unwrap();

    // feeders that feed rx_feeder
    // all these feeders need to output high signal in order to rx_feeder to trigger
    let rx_feed_feeders: Vec<String> = all_modules
        .modules
        .iter()
        .filter(|(_, val)| val.send_targets.contains(rx_feeder))
        .map(|(key, _)| key.to_owned())
        .collect();

    let mut feeding_map = FxHashMap::default();

    let mut break_cycle = false;

    let mut button_presses = 0;
    while !break_cycle {
        let mut module_queue = VecDeque::from(vec![("button".to_owned(), 0)]);
        button_presses += 1;

        while let Some((module_str, _)) = module_queue.pop_front() {
            let (send_sign, send_targets) = {
                let current_module = all_modules.modules.get_mut(&module_str).unwrap();
                if let Some(return_sng) = current_module.send() {
                    let targets = current_module.send_targets.clone();

                    (return_sng, targets)
                } else {
                    continue;
                }
            };

            for target_str in send_targets {
                if let Some(target_mod) = all_modules.modules.get_mut(&target_str) {
                    target_mod.receive(module_str.clone(), send_sign);

                    if rx_feed_feeders.contains(&target_str)
                        && !feeding_map.contains_key(&target_str)
                        && !send_sign
                    {
                        feeding_map.insert(target_str.clone(), button_presses);
                    }

                    module_queue.push_back((target_str, button_presses));
                }
            }

            if feeding_map.len() == rx_feed_feeders.len() {
                break_cycle = true;
            }
        }
    }

    feeding_map.into_values().reduce(lcm).unwrap()
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        let gcd = {
            let mut a2 = a;
            let mut b2 = b;
            while b2 != 0 {
                let temp = b2;
                b2 = a2 % b2;
                a2 = temp;
            }

            a2
        };

        a / gcd * b
    }
}
fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
