use data_structures::*;
use std::{
    env,
    io::{self, BufRead},
    ops,
    process::exit,
};

use seq_list::SeqList;

fn main() {
    use Error::*;
    if let Err(e) = actual_main() {
        let msg: String = match e {
            InputError => "Invalid input.".to_owned(),
            EmptyListError => "Empty list.".to_owned(),
            ArgsError(s) => format!("Invalid args: {}", s),
        };
        eprintln!("{}", msg);
        usage()
    };
}

fn usage() {
    println!(
        "Usage: {} -i/--implementation <name> | -h/--help",
        env::args().next().unwrap()
    )
}

fn stats<T>() -> Result<Stats<i32>>
where
    T: List<i32>,
    for<'b> &'b T: IntoIterator<Item = &'b i32>,
{
    let l = read_numbers().ok_or(Error::InputError)?;
    calc_stats(&l).ok_or(Error::EmptyListError)
}

fn actual_main() -> Result<()> {
    let implementation = get_list_implementation()
        .ok_or(Error::ArgsError("Specify the implementaion using `-i'."))?;

    let s = match implementation.as_ref() {
        "sequential" => stats::<SeqList<i32>>(),
        "singly_linked" => stats::<linked_list::LinkedList<i32>>(),
        "circular" => stats::<cir_linked_list::CirLinkedList<i32>>(),
        _ => Err(Error::ArgsError("No such list implementation.")),
    }?;

    println!("{:?}", s);
    Ok(())
}

fn get_list_implementation() -> Option<String> {
    let mut args = env::args();
    args.next()?;
    let option = args.next()?;
    if option == "-i" || option == "--implementation" {
        args.next()
    } else if option == "-h" || option == "--help" {
        usage();
        exit(0)
    } else {
        None
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

type Result<T> = std::result::Result<T, Error>;

enum Error {
    EmptyListError,
    ArgsError(&'static str),
    InputError,
}
