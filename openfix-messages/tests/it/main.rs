
//! Integration tests
//! 
//! Placed here so that they are only compiled once

#[cfg(test)]
mod deserialize;

#[cfg(test)]
mod serialize_empty_trailers;

#[cfg(test)]
mod serialize_with_trailers;

fn main() {
    println!("integration-tests");
}