#![feature(test)]

extern crate sample;
extern crate test  ;

use sample::world;
use test::Bencher;

#[bench]
fn world( b: &mut Bencher ) {
    b.iter( || {
        world::show()
    } );
}

