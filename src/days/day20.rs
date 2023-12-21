use std::collections::{HashMap, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1},
    multi::separated_list1,
    IResult,
};
use num::Integer;

#[derive(Debug, Clone)]
pub enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug)]
pub struct Module<'a> {
    module_type: ModuleType,
    module_name: &'a str,
    dst: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Message<'a> {
    src: &'a str,
    dst: &'a [&'a str],
    pulse: Pulse,
}

#[derive(Debug)]
pub struct StateMap<'a> {
    module_map: &'a HashMap<&'a str, Module<'a>>,
    flipflop_state: HashMap<&'a str, Pulse>,
    conj_state: HashMap<&'a str, HashMap<&'a str, Pulse>>,
    messages: VecDeque<Message<'a>>,
}

impl<'a> StateMap<'a> {
    pub fn new(module_map: &'a HashMap<&'a str, Module<'a>>) -> StateMap<'a> {
        let mut flipflop_state = HashMap::new();
        let mut conj_state = HashMap::new();

        for module in module_map.values() {
            for &dst_module_name in &module.dst {
                let dst_module = module_map.get(dst_module_name);
                if dst_module.is_none() {
                    continue;
                }

                let dst_module = dst_module.unwrap();
                if !matches!(dst_module.module_type, ModuleType::Conjunction) {
                    continue;
                }

                let inner_map = conj_state
                    .entry(dst_module.module_name)
                    .or_insert(HashMap::new());
                inner_map.insert(module.module_name, Pulse::Low);
            }

            match module.module_type {
                ModuleType::FlipFlop => flipflop_state.insert(module.module_name, Pulse::Low),
                _ => continue,
            };
        }

        let messages = VecDeque::new();

        StateMap {
            module_map,
            flipflop_state,
            conj_state,
            messages,
        }
    }
}

#[derive(Debug)]
pub struct Input<'a> {
    module_map: HashMap<&'a str, Module<'a>>,
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (input, type_str) = alt((tag("%"), tag("&"), tag("broadcaster")))(input)?;

    let module_type = match type_str {
        "%" => ModuleType::FlipFlop,
        "&" => ModuleType::Conjunction,
        "broadcaster" => ModuleType::Broadcaster,
        _ => unreachable!(),
    };

    let (input, mut module_name) = alpha0(input)?;

    if module_name.is_empty() {
        module_name = "broadcaster";
    }

    let (input, _) = tag(" -> ")(input)?;
    let (input, dst) = separated_list1(tag(", "), alpha1)(input)?;

    let module = Module {
        module_type,
        module_name,
        dst,
    };
    Ok((input, module))
}

pub fn parse_input(input: &str) -> Input {
    let mut module_map = HashMap::new();
    for line in input.lines() {
        match parse_module(line) {
            Ok((_, module)) => module_map.insert(module.module_name, module),
            _ => unreachable!(),
        };
    }

    Input { module_map }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut lows = 0;
    let mut highs = 0;
    let mut state = StateMap::new(&input.module_map);
    for i in 0..1000 {
        state.messages.push_back(Message {
            src: "button",
            dst: &["broadcaster"],
            pulse: Pulse::Low,
        });
        lows += 1;

        while let Some(message) = state.messages.pop_front() {
            let dst_modules = message
                .dst
                .iter()
                .filter_map(|&dst| state.module_map.get(dst));
            for module in dst_modules {
                match module.module_type {
                    ModuleType::Broadcaster => {
                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: message.pulse.clone(),
                        };

                        match new_message.pulse {
                            Pulse::Low => lows += new_message.dst.len(),
                            Pulse::High => highs += new_message.dst.len(),
                        }

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                    ModuleType::FlipFlop => {
                        if matches!(message.pulse, Pulse::High) {
                            continue;
                        }

                        let module_state =
                            state.flipflop_state.get_mut(module.module_name).unwrap();
                        match module_state {
                            Pulse::Low => {
                                *module_state = Pulse::High;
                            }
                            Pulse::High => {
                                *module_state = Pulse::Low;
                            }
                        }

                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: module_state.clone(),
                        };

                        match new_message.pulse {
                            Pulse::Low => lows += new_message.dst.len(),
                            Pulse::High => highs += new_message.dst.len(),
                        }

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                    ModuleType::Conjunction => {
                        let inner_map = state.conj_state.get_mut(module.module_name).unwrap();
                        inner_map
                            .entry(message.src)
                            .and_modify(|pulse| *pulse = message.pulse.clone());

                        let all_high = inner_map
                            .values()
                            .all(|pulse| matches!(*pulse, Pulse::High));
                        let pulse_value = match all_high {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: pulse_value,
                        };

                        match new_message.pulse {
                            Pulse::Low => lows += new_message.dst.len(),
                            Pulse::High => highs += new_message.dst.len(),
                        }

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                }
            }
        }
    }

    Some(lows as u64 * highs as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut state = StateMap::new(&input.module_map);
    let mut i: u64 = 0;
    let mut cycle_counter = HashMap::new();
    let mut cycles = HashMap::new();
    loop {
        i += 1;
        state.messages.push_back(Message {
            src: "button",
            dst: &["broadcaster"],
            pulse: Pulse::Low,
        });

        while let Some(message) = state.messages.pop_front() {
            let dst_modules = message
                .dst
                .iter()
                .filter_map(|&dst| state.module_map.get(dst));
            for module in dst_modules {
                match module.module_type {
                    ModuleType::Broadcaster => {
                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: message.pulse.clone(),
                        };

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                    ModuleType::FlipFlop => {
                        if matches!(message.pulse, Pulse::High) {
                            continue;
                        }

                        let module_state =
                            state.flipflop_state.get_mut(module.module_name).unwrap();
                        match module_state {
                            Pulse::Low => {
                                *module_state = Pulse::High;
                            }
                            Pulse::High => {
                                *module_state = Pulse::Low;
                            }
                        }

                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: module_state.clone(),
                        };

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                    ModuleType::Conjunction => {
                        let inner_map = state.conj_state.get_mut(module.module_name).unwrap();
                        inner_map
                            .entry(message.src)
                            .and_modify(|pulse| *pulse = message.pulse.clone());

                        if module.module_name == "tj" && matches!(message.pulse, Pulse::High) {
                            if !cycles.contains_key(message.src) {
                                match cycle_counter.get(message.src) {
                                    None => cycle_counter.insert(message.src, i),
                                    Some(u) => cycles.insert(message.src, i - u),
                                };
                            }

                            if cycles.len() == inner_map.len() {
                                let lcm = cycles.values().fold(1, |acc, v| acc.lcm(v));

                                return Some(lcm);
                            }
                        }

                        let all_high = inner_map
                            .values()
                            .all(|pulse| matches!(*pulse, Pulse::High));
                        let pulse_value = match all_high {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };
                        let new_message = Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: pulse_value,
                        };

                        state.messages.push_back(Message {
                            src: module.module_name,
                            dst: &module.dst,
                            pulse: new_message.pulse.clone(),
                        });
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day20/test.txt");
    const TEST_INPUT2: &str = include_str!("../../input/day20/test2.txt");
    #[test]
    fn test_day20_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(32000000));
    }

    #[test]
    fn test_day20_part1_test2() {
        let input = parse_input(TEST_INPUT2);

        let resp = part1(&input);

        assert_eq!(resp, Some(11687500));
    }

    #[test]
    fn test_day20_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
