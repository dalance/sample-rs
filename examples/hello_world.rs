extern crate sample;

use sample::{hello, world};

pub fn main() {
    println!( "{} {}", hello::show(), world::show() )
}
