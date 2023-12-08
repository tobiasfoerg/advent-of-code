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
