use std::collections::BTreeMap;

use p2::p2;

// type Point = (i32, i32);
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);
type BoundedSet = BTreeMap<usize, Vec<Point>>;
mod p1;
mod p2;

fn main() {
    // p1();
    p2();
}
