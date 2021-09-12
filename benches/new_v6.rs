#![feature(test)]

extern crate test;
extern crate uuidv6_rs;

use test::Bencher;
use uuidv6_rs::iso::new_v6;

#[bench]
fn bench_new_v6(b: &mut Bencher) {
    b.iter(|| new_v6());
}