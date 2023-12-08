#![feature(test)]
extern crate test;

mod part_one;
mod part_two;

use anyhow::Result;

use lazy_static::lazy_static;
use part_one::part_one;
use part_two::part_two;

lazy_static! {
    pub(crate) static ref INPUT: String = include_str!("../input.txt").to_string();
}

pub fn main() -> Result<()> {
    let part_one_result = part_one(&INPUT);
    let part_two_result = part_two(&INPUT);
    println!("Part 1: {part_one_result}");
    println!("Part 2: {part_two_result}");
    Ok(())
}

pub(crate) fn parse_game_to_sets(game: &str) -> Vec<(usize, usize, usize)> {
    game.split_once(": ")
        .unwrap()
        .1
        .split(";")
        .map(|set| {
            let (mut red, mut blue, mut green) = (0, 0, 0);
            set.split(", ").for_each(|item| {
                let (amount, color) = item.trim().split_once(" ").unwrap();
                match color {
                    "red" => red += amount.parse::<usize>().unwrap(),
                    "blue" => blue += amount.parse::<usize>().unwrap(),
                    "green" => green += amount.parse::<usize>().unwrap(),
                    _ => panic!("Invalid color"),
                }
            });
            (red, blue, green)
        })
        .collect()
}
