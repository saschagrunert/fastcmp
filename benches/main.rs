#![feature(test)]
extern crate test;
extern crate fastcmp;

use fastcmp::Compare;

#[bench]
fn slice_compare(b: &mut test::Bencher) {
    let vec1 = vec![5; 256];
    let vec2 = vec1.clone();

    b.iter(|| {
        let (s1, s2) = (&vec1, &vec2);
        s1 == s2
    });
    b.bytes = vec1.len() as u64;
}

#[bench]
fn fast_compare(b: &mut test::Bencher) {
    let vec1 = vec![5; 256];
    let vec2 = vec1.clone();

    b.iter(|| {
        let (s1, s2) = (&vec1, &vec2);
        s1.feq(s2)
    });
    b.bytes = vec1.len() as u64;
}
