use crate::{OrderRule, Update};

use nom::{
    character::complete::{char, line_ending, u32},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::{pair, separated_pair},
    Finish, IResult,
};

fn parse_order_rule(input: &str) -> IResult<&str, OrderRule> {
    map(separated_pair(u32, char('|'), u32), |(before, after)| {
        OrderRule { before, after }
    })(input)
}

fn parse_ordering_rule_list(input: &str) -> IResult<&str, Vec<OrderRule>> {
    separated_list0(line_ending, parse_order_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    separated_list0(char(','), u32)(input)
}

fn parse_update_list(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list0(line_ending, parse_update)(input)
}

pub(super) fn parse_input(
    input: &str,
) -> Result<(Vec<OrderRule>, Vec<Update>), nom::error::Error<&str>> {
    let mut parse_all = all_consuming(separated_pair(
        parse_ordering_rule_list,
        pair(line_ending, line_ending),
        parse_update_list,
    ));

    parse_all(input)
        .finish()
        .map(|(_, (rules, updates))| (rules, updates))
}
