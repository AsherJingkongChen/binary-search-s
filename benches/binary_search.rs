#![feature(test)]

extern crate test;
extern crate rand;

use rand::Rng;
use test::black_box;
use test::Bencher;
use binary_search_s::*;
use rand::thread_rng;

enum Cache {
  L1,
  L2,
  L3,
}

impl Cache {
  fn size(&self) -> usize {
    match self {
      Cache::L1 => 1000,    // 8kb
      Cache::L2 => 10_000,  // 80kb
      Cache::L3 => 1_000_000, // 8Mb
    }
  }
}

fn binary_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
where
  F: Fn(usize) -> usize,
{
  let size = cache.size();
  let v = (0..size).map(&mapper).collect::<Vec<_>>();
  let mut r = 0usize;
  b.iter(move || {
    // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
    r = r.wrapping_mul(1664525).wrapping_add(1013904223);
    // Lookup the whole range to get 50% hits and 50% misses.
    let i = mapper(r % size);
    black_box(v.binary_search_s(&i).unwrap());
  });
}

fn binary_search_worst_case(b: &mut Bencher, cache: Cache) {
  let size = cache.size();

  let mut v = vec![0; size];
  let i = 1;
  v[size - 1] = i;
  b.iter(move || {
    black_box(v.binary_search_s(&i).unwrap());
  });
}

#[bench]
fn binary_search_l1(b: &mut Bencher) {
  let multiple = (thread_rng().gen::<u8>() as usize).max(1);
  binary_search(b, Cache::L1, |i| i * multiple);
}

#[bench]
fn binary_search_l2(b: &mut Bencher) {
  let multiple = (thread_rng().gen::<u8>() as usize).max(1);
  binary_search(b, Cache::L2, |i| i * multiple);
}

#[bench]
fn binary_search_l3(b: &mut Bencher) {
  let multiple = (thread_rng().gen::<u8>() as usize).max(1);
  binary_search(b, Cache::L3, |i| i * multiple);
}

// Other
// test binary_search_l1            ... bench:          67 ns/iter (+/- 17)
// test binary_search_l1_with_dups  ... bench:          43 ns/iter (+/- 12)
// test binary_search_l1_worst_case ... bench:          16 ns/iter (+/- 2)
// test binary_search_l2            ... bench:          93 ns/iter (+/- 31)
// test binary_search_l2_with_dups  ... bench:          63 ns/iter (+/- 20)
// test binary_search_l2_worst_case ... bench:          23 ns/iter (+/- 6)
// test binary_search_l3            ... bench:         322 ns/iter (+/- 118)
// test binary_search_l3_with_dups  ... bench:         243 ns/iter (+/- 184)
// test binary_search_l3_worst_case ... bench:          42 ns/iter (+/- 19)

// Me
// test binary_search_l1            ... bench:          66 ns/iter (+/- 39)
// test binary_search_l1_with_dups  ... bench:          49 ns/iter (+/- 48)
// test binary_search_l1_worst_case ... bench:           9 ns/iter (+/- 1)
// test binary_search_l2            ... bench:          81 ns/iter (+/- 27)
// test binary_search_l2_with_dups  ... bench:          65 ns/iter (+/- 9)
// test binary_search_l2_worst_case ... bench:          11 ns/iter (+/- 1)
// test binary_search_l3            ... bench:         293 ns/iter (+/- 37)
// test binary_search_l3_with_dups  ... bench:         232 ns/iter (+/- 43)
// test binary_search_l3_worst_case ... bench:          18 ns/iter (+/- 10)

#[bench]
fn binary_search_l1_with_dups(b: &mut Bencher) {
  binary_search(b, Cache::L1, |i| i / 16 * 16);
}

#[bench]
fn binary_search_l2_with_dups(b: &mut Bencher) {
  binary_search(b, Cache::L2, |i| i / 16 * 16);
}

#[bench]
fn binary_search_l3_with_dups(b: &mut Bencher) {
  binary_search(b, Cache::L3, |i| i / 16 * 16);
}

#[bench]
fn binary_search_l1_worst_case(b: &mut Bencher) {
  binary_search_worst_case(b, Cache::L1);
}

#[bench]
fn binary_search_l2_worst_case(b: &mut Bencher) {
  binary_search_worst_case(b, Cache::L2);
}

#[bench]
fn binary_search_l3_worst_case(b: &mut Bencher) {
  binary_search_worst_case(b, Cache::L3);
}
