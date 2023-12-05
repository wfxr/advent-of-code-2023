#![feature(iter_intersperse)]
#![feature(array_windows)]
#![feature(extract_if)]
#![feature(test)]
#![allow(clippy::get_first)]
extern crate test;

mod macros;
#[cfg(test)]
mod testmacros;

#[rustfmt::skip]
crate::aoc!(
    day01,
    day02,
); // +SOLUTIONS+

pub type AocResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Solution {
    pub part1: fn(&str) -> AocResult<String>,
    pub part2: fn(&str) -> AocResult<String>,
    pub input: &'static str,
}
