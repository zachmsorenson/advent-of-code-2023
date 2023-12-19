use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, digit1},
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub enum Op {
    LessThan,
    GreaterThan,
    None,
}

#[derive(Debug)]
pub struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    pub fn get(&self, c: char) -> i32 {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct PartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl PartRange {
    pub fn get(&self, attr: char) -> (u64, u64) {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => (0, 0),
        }
    }

    pub fn set(&mut self, attr: char, value: (u64, u64)) {
        match attr {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => unreachable!(),
        }
    }

    pub fn from_break(mut self, attr: char, value: u64) -> (Option<PartRange>, Option<PartRange>) {
        let (l, h) = self.get(attr);
        if value < l {
            return (None, Some(self));
        }
        if h < value {
            return (Some(self), None);
        }

        let mut other = PartRange {
            x: self.x,
            m: self.m,
            a: self.a,
            s: self.s,
        };
        self.set(attr, (l, value - 1));
        other.set(attr, (value, h));

        (Some(self), Some(other))
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    attr: char,
    op: Op,
    value: i32,
    to: &'a str,
}

#[derive(Debug)]
pub struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
pub struct Input<'a> {
    start: String,
    workflow_map: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, s) = alpha1(input)?;

    if input.is_empty() {
        let rule = Rule {
            attr: 'N',
            op: Op::None,
            value: 0,
            to: s,
        };
        return Ok((input, rule));
    }

    let (input, op) = anychar(input)?;
    let op = match op {
        '>' => Op::GreaterThan,
        '<' => Op::LessThan,
        _ => unreachable!(),
    };

    let (input, value) = digit1(input)?;
    let value = value.parse().unwrap();

    let (input, to) = preceded(tag(":"), alpha1)(input)?;

    let rule = Rule {
        attr: s.chars().next().unwrap(),
        op,
        value,
        to,
    };

    Ok((input, rule))
}

pub fn parse_input(input: &str) -> Input {
    let mut workflow_map = HashMap::new();

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let line = line.strip_suffix('}').unwrap();
        let mut split = line.split('{');
        let workflow_name = split.next().unwrap();
        let line = split.next().unwrap();

        let mut rules = Vec::new();
        let rule_splits = line.split(',');
        for rule_split in rule_splits {
            if let Ok((_, rule)) = parse_rule(rule_split) {
                rules.push(rule);
            } else {
                unreachable!()
            }
        }

        let workflow = Workflow { rules };
        workflow_map.insert(workflow_name, workflow);
    }

    let mut parts = Vec::new();
    for line in lines {
        let line = line.strip_prefix('{').unwrap();
        let line = line.strip_suffix('}').unwrap();

        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for split in line.split(',') {
            let mut chars = split.chars();
            let attr = chars.next().unwrap();
            chars.next();

            let mut value = 0;
            for c in chars {
                if c.is_ascii_digit() {
                    value = 10 * value + c as i32 - '0' as i32;
                }
            }

            match attr {
                'x' => part.x = value,
                'm' => part.m = value,
                'a' => part.a = value,
                's' => part.s = value,
                _ => unreachable!(),
            }
        }

        parts.push(part);
    }

    Input {
        start: "in".to_string(),
        workflow_map,
        parts,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut sum = 0;
    for part in &input.parts {
        let mut curr = input.start.as_str();
        loop {
            if curr == "A" {
                sum += part.x;
                sum += part.m;
                sum += part.a;
                sum += part.s;
                break;
            }
            if curr == "R" {
                break;
            }

            let workflow = input.workflow_map.get(curr).unwrap();

            for rule in &workflow.rules {
                let matches = match rule.op {
                    Op::LessThan => part.get(rule.attr) < rule.value,
                    Op::GreaterThan => part.get(rule.attr) > rule.value,
                    Op::None => true,
                };

                if matches {
                    curr = rule.to;
                    break;
                }
            }
        }
    }

    Some(sum as u64)
}

fn do_ranges(curr: &str, workflow_map: &HashMap<&str, Workflow>, part_range: PartRange) -> u64 {
    if curr == "A" {
        let mut prod = 1;
        prod *= part_range.x.1 - part_range.x.0 + 1;
        prod *= part_range.m.1 - part_range.m.0 + 1;
        prod *= part_range.a.1 - part_range.a.0 + 1;
        prod *= part_range.s.1 - part_range.s.0 + 1;
        return prod;
    }
    if curr == "R" {
        return 0;
    }

    let mut sum = 0;
    let mut curr_range = part_range;
    let workflow = workflow_map.get(curr).unwrap();
    for rule in &workflow.rules {
        // Split range based on the rule.
        let (matched_range, unmatched_range) = match rule.op {
            Op::LessThan => {
                let (range1, range2) = curr_range.from_break(rule.attr, rule.value as u64);
                (range1, range2)
            }
            Op::GreaterThan => {
                let (range1, range2) = curr_range.from_break(rule.attr, rule.value as u64 + 1);
                (range2, range1)
            }
            Op::None => (Some(curr_range), None),
        };

        if let Some(matched_range) = matched_range {
            sum += do_ranges(rule.to, workflow_map, matched_range);
        }
        if let Some(unmatched_range) = unmatched_range {
            curr_range = unmatched_range;
        } else {
            break;
        }
    }

    sum
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let curr = "in";
    let range = PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };

    let sum = do_ranges(curr, &input.workflow_map, range);

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day19/test.txt");
    #[test]
    fn test_day19_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(19114));
    }

    #[test]
    fn test_day19_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(167409079868000));
    }
}
