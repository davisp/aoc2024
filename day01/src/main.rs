use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error parsing integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;

fn solve1(input: &str) -> Result<usize> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let row = line.split_whitespace().collect::<Vec<_>>();
        assert_eq!(row.len(), 2);

        let l: usize = row[0].parse()?;
        left.push(l);

        let r: usize = row[1].parse()?;
        right.push(r);
    }

    left.sort();
    right.sort();

    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| std::cmp::max(l, r) - std::cmp::min(l, r))
        .sum();

    Ok(sum)
}

fn solve2(input: &str) -> Result<usize> {
    let mut keys: Vec<usize> = Vec::new();
    let mut occurs: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let row = line.split_whitespace().collect::<Vec<_>>();
        assert_eq!(row.len(), 2);

        let key = row[0].parse()?;
        let val = row[1].parse()?;

        keys.push(key);
        *occurs.entry(val).or_insert(0) += 1;
    }

    let sum = keys
        .iter()
        .map(|key| {
            if let Some(count) = occurs.get(key) {
                key * count
            } else {
                0
            }
        })
        .sum();

    Ok(sum)
}

fn main() -> Result<()> {
    let data = if std::fs::exists("data/sample")? {
        std::fs::read_to_string("data/sample")?
    } else {
        "".to_string()
    };

    let result = solve1(&data);
    println!("Sample 1: {:?}", result);
    assert!(matches!(result, Ok(11)));

    let result = solve2(&data);
    println!("Sample 2: {:?}", result);
    assert!(matches!(result, Ok(31)));

    let data = if std::fs::exists("data/input")? {
        std::fs::read_to_string("data/input")?
    } else {
        "".to_string()
    };

    let result = solve1(&data);
    println!("Answer 1: {:?}", result);

    let result = solve2(&data);
    println!("Answer 2: {:?}", result);

    Ok(())
}
