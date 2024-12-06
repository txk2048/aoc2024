use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use clap::{command, value_parser, Arg};

mod parser;

type Page = u32;
type Update = Vec<Page>;

struct OrderRule {
    before: Page,
    after: Page,
}

fn is_sorted(update: &Update, rules: &[OrderRule]) -> bool {
    let mut edges = rules
        .iter()
        .filter(|rule| update.contains(&rule.before) && update.contains(&rule.after))
        .fold(HashMap::new(), |mut m, rule| {
            m.entry(rule.after)
                .or_insert(HashSet::new())
                .insert(rule.before);

            m
        });

    for p in update {
        // we have found a page with incoming edges
        if edges.contains_key(p) {
            return false;
        }

        // remove all incoming edges from p
        for (_, e) in edges.iter_mut() {
            e.retain(|q| q != p);
        }

        // remove any nodes with no incoming edges
        edges.retain(|_, e| !e.is_empty());
    }

    true
}

fn sort(update: &Update, rules: &[OrderRule]) -> Option<Update> {
    let mut edges = rules
        .iter()
        .filter(|rule| update.contains(&rule.before) && update.contains(&rule.after))
        .fold(HashMap::new(), |mut m, rule| {
            m.entry(rule.after)
                .or_insert(HashSet::new())
                .insert(rule.before);

            m
        });

    // pages with no incoming edges
    let mut entries: Vec<_> = update
        .iter()
        .copied()
        .filter(|page| !edges.contains_key(page))
        .collect();

    let mut sorted_update = Vec::new();

    while let Some(p) = entries.pop() {
        sorted_update.push(p);

        // remove any incoming edges from p
        for (_, e) in edges.iter_mut() {
            e.retain(|q| *q != p);
        }

        let now_insertable = edges.iter().filter(|(_, e)| e.is_empty()).map(|(q, _)| *q);
        entries.extend(now_insertable);

        // remove inserted edges
        edges.retain(|_, e| !e.is_empty())
    }

    if edges.is_empty() {
        Some(sorted_update)
    } else {
        println!("{:?}", edges);
        None
    }
}

fn part1(updates: &[Update], rules: &[OrderRule]) -> u32 {
    updates
        .iter()
        .filter(|update| is_sorted(update, rules))
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum()
}

fn part2(updates: &[Update], rules: &[OrderRule]) -> u32 {
    updates
        .iter()
        .filter(|update| !is_sorted(update, &rules))
        .map(|update| {
            sort(update, &rules).map(|u| {
                let mid = u.len() / 2;
                u[mid]
            })
        })
        .sum::<Option<_>>()
        .expect("could not sort")
}

fn main() {
    let matches = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("input").unwrap();
    let input_contents = fs::read_to_string(input_path).expect("could not read input");
    let (rules, updates) =
        parser::parse_input(input_contents.trim()).expect("could not parse input");

    let result1 = part1(&updates, &rules);
    println!("Part 1: {}", result1);

    let result2 = part2(&updates, &rules);
    println!("Part 2: {}", result2);
}
