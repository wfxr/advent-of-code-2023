#![feature(iter_intersperse)]
#![feature(iter_array_chunks)]
#![feature(iterator_try_collect)]
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
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
); // +SOLUTIONS+

pub use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

pub use anyhow::{anyhow, bail, ensure, Error, Result};
pub use itertools::Itertools;

pub struct Solution {
    pub part1: fn(&str) -> Result<String>,
    pub part2: fn(&str) -> Result<String>,
    pub input: &'static str,
}
