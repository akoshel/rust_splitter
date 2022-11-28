// use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::{Eq, Ord, PartialEq};

pub fn eq_operator<T: Eq>(item1: &T, item2: &T) -> bool {
    item1 == item2
}

pub fn partial_eq<T: PartialEq>(item1: &T, item2: &T) -> bool {
    item1 == item2
}

pub fn ge<T: Ord>(item1: &T, item2: &T) -> bool {
    item1 >= item2
}
pub fn gt<T: Ord>(item1: &T, item2: &T) -> bool {
    item1 > item2
}
