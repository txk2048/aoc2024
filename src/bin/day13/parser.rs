use crate::{Machine, Vec2};

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u64};
use nom::combinator::{all_consuming, map};
use nom::multi::count;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{Finish, IResult};

fn parse_button_a(input: &str) -> IResult<&str, Vec2> {
    map(
        preceded(
            tag("Button A: "),
            separated_pair(
                preceded(tag("X+"), u64),
                tag(", "),
                preceded(tag("Y+"), u64),
            ),
        ),
        |(x, y)| Vec2 { x, y },
    )(input)
}

fn parse_button_b(input: &str) -> IResult<&str, Vec2> {
    map(
        preceded(
            tag("Button B: "),
            separated_pair(
                preceded(tag("X+"), u64),
                tag(", "),
                preceded(tag("Y+"), u64),
            ),
        ),
        |(x, y)| Vec2 { x, y },
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, Vec2> {
    map(
        preceded(
            tag("Prize: "),
            separated_pair(
                preceded(tag("X="), u64),
                tag(", "),
                preceded(tag("Y="), u64),
            ),
        ),
        |(x, y)| Vec2 { x, y },
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        tuple((
            terminated(parse_button_a, line_ending),
            terminated(parse_button_b, line_ending),
            parse_prize,
        )),
        |(button_a, button_b, prize)| Machine {
            button_a,
            button_b,
            prize,
        },
    )(input)
}

pub fn parse_machines(input: &str) -> Result<Vec<Machine>, nom::error::Error<String>> {
    all_consuming(separated_list0(count(line_ending, 2), parse_machine))(input)
        .map_err(|e| e.to_owned())
        .finish()
        .map(|(_, machines)| machines)
}
