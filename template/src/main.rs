#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Not Implemented")]
    NotImplemented,
    #[error("Error parsing integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

type Result<T> = std::result::Result<T, Error>;

fn solve1(_input: &str) -> Result<()> {
    Err(Error::NotImplemented)
}

fn solve2(_input: &str) -> Result<()> {
    Err(Error::NotImplemented)
}

fn main() -> Result<()> {
    let data = if std::fs::exists("data/sample")? {
        std::fs::read_to_string("data/sample")?
    } else {
        "".to_string()
    };

    let result = solve1(&data);
    println!("Sample 1: {:?}", result);
    //assert!(matches!(result, Ok(_)));

    let result = solve2(&data);
    println!("Sample 2: {:?}", result);
    //assert!(matches!(result, Ok(_)));

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
