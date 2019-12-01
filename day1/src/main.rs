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
    let mut sum = 0;
    for l in input.lines() {
        let x = l.parse::<i32>()?;
        sum += x / 3 - 2;
    }
    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut sum = 0;
    for l in input.lines() {
        sum += fuel(l.parse::<i32>()?);
    }
    println!("{}", sum);
    Ok(())
}

fn fuel(x: i32) -> i32 {
    let fuel_required = x / 3 - 2;
    match fuel_required {
        std::i32::MIN..=0 => 0,
        _ => fuel_required + fuel(fuel_required),
    }
}
