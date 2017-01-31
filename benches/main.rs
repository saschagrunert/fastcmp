#![feature(test)]
extern crate test;

extern crate fastcmp;
use fastcmp::Compare;

#[bench]
fn slice_compare_equal(b: &mut test::Bencher) {
    let vec1 = vec![5; 256];
    let vec2 = vec1.clone();

    b.iter(|| {
        assert!(&vec2 == &vec1);
    });
    b.bytes = vec1.len() as u64;
}

#[bench]
fn fast_compare_equal(b: &mut test::Bencher) {
    let vec1 = vec![5; 256];
    let vec2 = vec1.clone();

    b.iter(|| {
        assert!(vec1.feq(&vec2));
    });
    b.bytes = vec1.len() as u64;
}

#[bench]
fn slice_compare_unequal(b: &mut test::Bencher) {
    let vec = vec![5; 255];
    let mut vec1 = vec.clone();
    let mut vec2 = vec.clone();
    vec1.push(1);
    vec2.push(2);

    b.iter(|| assert!(&vec1 != &vec2));
    b.bytes = vec1.len() as u64;
}

#[bench]
fn fast_compare_unequal(b: &mut test::Bencher) {
    let vec = vec![5; 255];
    let mut vec1 = vec.clone();
    let mut vec2 = vec.clone();
    vec1.push(1);
    vec2.push(2);

    b.iter(|| assert!(!vec1.feq(&vec2)));
    b.bytes = vec1.len() as u64;
}
