#![feature(iter_intersperse)]
#![feature(array_windows)]
#![feature(extract_if)]
#![feature(iter_array_chunks)]
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
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
); // +SOLUTIONS+

pub use anyhow::{anyhow, bail, ensure, Error, Result};

pub struct Solution {
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
    pub input: &'static str,
}
