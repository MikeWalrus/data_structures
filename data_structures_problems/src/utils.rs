use data_structures::*;
use std::io::{self, BufRead};

pub fn read_numbers<T, L>() -> Option<L>
where
    T: std::str::FromStr,
    L: List<T>,
    for<'b> &'b L: IntoIterator<Item = &'b T>,
{
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let num_list: Option<L> = lines
        .flat_map(|line| {
            line.unwrap_or_default()
                .split_whitespace()
                .map(|s| s.parse::<T>().ok())
                .collect::<Vec<_>>()
        })
        .collect();
    num_list
}