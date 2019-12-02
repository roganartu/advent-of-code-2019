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
    for line in input.lines() {
        let mut codes: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        codes[1] = 12;
        codes[2] = 2;
        println!("pos 0: {}", computer(&mut codes.clone()));
    }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    for line in input.lines() {
        let mut codes: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        'done: for noun in 0..=99 {
            for verb in 0..=99 {
                codes[1] = noun;
                codes[2] = verb;
                if computer(&mut codes.clone()) == 19690720 {
                    println!(
                        "noun: {} | verb: {} | ans: {}",
                        noun,
                        verb,
                        100 * noun + verb
                    );
                    break 'done;
                }
            }
        }
    }
    Ok(())
}

fn computer(codes: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    while i < codes.len() {
        match codes[i] {
            1 => {
                let x = codes[i + 1] as usize;
                let y = codes[i + 2] as usize;
                let z = codes[i + 3] as usize;
                codes[z] = codes[x] + codes[y];
                i += 3;
            }
            2 => {
                let x = codes[i + 1] as usize;
                let y = codes[i + 2] as usize;
                let z = codes[i + 3] as usize;
                codes[z] = codes[x] * codes[y];
                i += 3;
            }
            99 => break,
            _ => println!("unknown op"),
        }
        i += 1;
    }
    return codes[0];
}
