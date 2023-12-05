use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, none_of},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Game {
    cubes: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

fn parse_pair(input: &str) -> IResult<&str, (u64, &str)> {
    let (input, (n, color)) = separated_pair(digit1, tag(" "), alpha1)(input)?;

    let n = n.parse().unwrap();

    Ok((input, (n, color)))
}

fn parse_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, pairs) = separated_list1(tag(", "), parse_pair)(input)?;

    let mut cube_set = CubeSet {
        red: 0,
        green: 0,
        blue: 0,
    };
    for (n, color) in pairs {
        match color {
            "red" => cube_set.red = n,
            "blue" => cube_set.blue = n,
            "green" => cube_set.green = n,
            _ => unreachable!(),
        }
    }
    Ok((input, cube_set))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;

    Ok((input, Game { cubes: sets }))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, _) = preceded(many1(none_of(":")), tag(": "))(input)?;
    let (input, game) = parse_game(input)?;

    Ok((input, game))
}

pub fn parse_input(input: &str) -> Vec<Game> {
    let (_, games) = separated_list1(newline, parse_line)(input).unwrap();

    games
}

pub fn part1(input: &[Game]) -> Option<u64> {
    let mut sum = 0;
    for (i, game) in input.iter().enumerate() {
        let mut good = true;
        for set in &game.cubes {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                good = false;
                break;
            }
        }
        if good {
            sum += i as u64 + 1;
        }
    }

    Some(sum)
}

pub fn part2(input: &[Game]) -> Option<u64> {
    let mut sum = 0;
    for game in input {
        let mut max_set = CubeSet {
            red: 0,
            blue: 0,
            green: 0,
        };
        for set in &game.cubes {
            max_set.red = u64::max(max_set.red, set.red);
            max_set.green = u64::max(max_set.green, set.green);
            max_set.blue = u64::max(max_set.blue, set.blue);
        }

        sum += max_set.red * max_set.green * max_set.blue;
    }

    Some(sum)
}
