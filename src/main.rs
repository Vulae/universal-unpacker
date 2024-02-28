#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;

pub mod util;
pub mod extract;
mod cli;



fn main() -> Result<(), Box<dyn Error>> {
    cli::execute_cli()?;
    Ok(())
}


