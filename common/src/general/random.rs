///! Lol I don't like to use rand, so I would like to use getrandom instead
use getrandom::{fill};
use anyhow::{bail, Result};
pub fn random_from_range(range: u32) -> Result<u32> {
    let limit = u32::MAX - (u32::MAX % range);

    loop {
        let n = getrandom::u32()?;
        if n <= limit {
            return Ok(n % range);
        }
        
    }
}