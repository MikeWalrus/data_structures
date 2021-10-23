use data_structures::*;
use std::{
    io::{self, BufRead},
    ops,
    process::exit,
};

use seq_list::SeqList;

fn main() {
    let l = read_numbers::<SeqList<i32>>().unwrap_or_else(|| {
        eprintln!("error: Non-integer input.");
        exit(1)
    });

    for i in l.iter() {
        print!("{} ", i);
    }

    println!();

    match calc_stats(&l) {
        Some(stats) => println!("{:?}", stats),
        _ => println!("Empty list."),
    }
}

fn read_numbers<L>() -> Option<L>
where
    L: List<i32>,
    for<'b> &'b L: IntoIterator<Item = &'b i32>,
{
    let stdin = io::stdin();
    let handle = stdin.lock();
    let lines = handle.lines();
    let num_list: Option<L> = lines
        .flat_map(|line| {
            line.unwrap_or_default()
                .split_whitespace()
                .map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect();
    num_list
}

#[derive(Debug)]
struct Stats<T> {
    min: T,
    max: T,
    avg: f64,
}

fn calc_stats<'a, T, L>(l: &'a L) -> Option<Stats<T>>
where
    T: 'a + Clone + PartialOrd + ops::Add<&'a T, Output = T> + Into<f64>,
    L: List<T>,
    for<'b> &'b L: IntoIterator<Item = &'b T>,
{
    let i = &mut l.into_iter();
    let first_item = i.next()?;
    let (min, max, sum, len) = i.fold(
        (
            first_item.clone(),
            first_item.clone(),
            first_item.clone(),
            0_usize,
        ),
        |(min, max, sum, len), i: &'a T| {
            let new_min;
            let new_max;

            if i < &min {
                new_min = i.clone();
            } else {
                new_min = min;
            }

            if i > &max {
                new_max = i.clone();
            } else {
                new_max = max;
            }

            (new_min, new_max, sum + i, len + 1)
        },
    );
    Some(Stats {
        min,
        max,
        avg: sum.into() / (len as f64),
    })
}
