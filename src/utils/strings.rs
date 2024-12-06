use std::{error::Error, fmt};

#[derive(Debug)]
pub struct StringUtils;

impl Error for StringUtils {}

impl fmt::Display for StringUtils {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input provided")
    }
}


pub fn split_from(input: &str, location: usize) -> Result<(&str, &str), StringUtils> {
    if input.len() < 3 || location > input.len() - 1 {
        return Err(StringUtils);
    }
    let (left, _) = input.split_at(location + 1);
    let (_, right) = input.split_at(location);
    return Ok((left, right));
}