use data_structures::*;
use std::{
    env, ops,
    process::exit,
    time::{Duration, Instant},
};

use seq_list::SeqList;
mod utils;

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
    let l = utils::read_numbers().ok_or(Error::InputError)?;
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

#[derive(Debug)]
#[allow(dead_code)]
struct Stats<T> {
    min: T,
    max: T,
    avg: f64,
    time: Duration,
}

fn calc_stats<'a, T, L>(l: &'a L) -> Option<Stats<T>>
where
    T: 'a + Clone + Ord + ops::Add<&'a T, Output = T> + Into<f64>,
    L: List<T>,
    for<'b> &'b L: IntoIterator<Item = &'b T>,
{
    let start = Instant::now();
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

            new_min = std::cmp::min(i, &min);
            new_max = std::cmp::max(i, &max);

            (new_min.clone(), new_max.clone(), sum + i, len + 1)
        },
    );
    Some(Stats {
        min,
        max,
        avg: sum.into() / (len as f64),
        time: start.elapsed(),
    })
}

type Result<T> = std::result::Result<T, Error>;

enum Error {
    EmptyListError,
    ArgsError(&'static str),
    InputError,
}
