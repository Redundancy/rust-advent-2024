use anyhow::anyhow;
use itertools::Itertools;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let f = std::fs::File::open("data/one.txt")?;
    let c = io::BufReader::new(f);

    let line_to_tuple = |s: String| -> anyhow::Result<(usize, usize)> {
        let mut items = s.split_whitespace().map(|item| usize::from_str(item));
        match (items.next(), items.next(), items.next()) {
            (Some(first), Some(second), None) => Ok((first?, second?)),
            _ => Err(anyhow!("Invalid input: {}", s)),
        }
    };

    let (mut first, mut second) = c.lines().map_ok(line_to_tuple).flatten_ok().fold_ok(
        (Vec::new(), Vec::new()),
        |(mut a, mut b), (a_next, b_next)| {
            a.push(a_next);
            b.push(b_next);
            (a, b)
        },
    )?;

    first.sort();
    second.sort();

    println!("{}", distance(&first, &second));
    Ok(())
}

fn distance(v1: &Vec<usize>, v2: &Vec<usize>) -> usize {
    v1.iter().zip(v2.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}
