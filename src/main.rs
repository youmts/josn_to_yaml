use json_to_yaml::json_to_yaml;

use std::io::{self, Read};

use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] io::Error),
    #[error("lib error")]
    Lib(#[from] json_to_yaml::LibError),
}

fn main() -> Result<(), Error> {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input)?;

    let output = json_to_yaml(input)?;

    println!("{}", output);

    Ok(())
}
