#[macro_use]
extern crate simple_error;

use std::cmp;
use std::i32;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    println!("{}", count(input, -1).expect("something went wrong"));
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    println!("{}", count(input, 2).expect("something went wrong"));
    Ok(())
}

fn count(input: &str, limit: i32) -> Result<i32> {
    let range: Vec<i32> = input
        .trim()
        .split('-')
        .map(|x| x.parse::<i32>().expect("bad number format"))
        .collect();

    if range.len() != 2 {
        bail!(format!("Expected 2 elements, got {}", range.len(),));
    }

    let (mut code, end) = (range[0], range[1]);

    let mut count = 0;
    while code <= end {
        if valid(code.to_string(), limit) {
            count += 1;
        }
        code += 1;
    }
    Ok(count)
}

fn valid(code: String, limit: i32) -> bool {
    let mut last = code.chars().next().unwrap();
    let mut dup_count = 0;
    let mut min_dupes = i32::MAX;
    for c in code.chars().skip(1) {
        if c == last {
            dup_count += 1;
            continue;
        }
        if dup_count > 0 {
            min_dupes = cmp::min(min_dupes, dup_count + 1);
        }
        dup_count = 0;
        if c < last {
            return false;
        }
        last = c;
    }

    if dup_count > 0 {
        min_dupes = cmp::min(min_dupes, dup_count + 1);
    }

    (limit < 0 && min_dupes >= 2) || (limit > 0 && min_dupes == limit)
}
