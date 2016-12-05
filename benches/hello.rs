#![feature(test)]

extern crate sample;
extern crate test  ;

use sample::hello;
use test::Bencher;

#[bench]
fn hello( b: &mut Bencher ) {
    b.iter( || {
        hello::show()
    } );
}

