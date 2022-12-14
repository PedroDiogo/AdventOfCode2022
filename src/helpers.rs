/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::{collections::HashSet, ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

pub trait First<T> {
    fn first(&self) -> Option<&T>;
}
impl<T> First<T> for HashSet<T> {
    fn first(&self) -> Option<&T> {
        if self.len() > 0 {
            return self.iter().next();
        }
        None
    }
}

pub trait Overlaps<T: PartialOrd> {
    fn overlaps(&self, other_range: &RangeInclusive<T>) -> bool;
}

impl<T: PartialOrd> Overlaps<T> for RangeInclusive<T> {
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        !(self.start() > other.end() || other.start() > self.end())
    }
}

pub trait FullyContains<T: Copy + PartialOrd + std::ops::Sub<Output = T>> {
    fn fully_contains(&self, other_range: &RangeInclusive<T>) -> bool;
}

impl<T: Copy + PartialOrd + std::ops::Sub<Output = T>> FullyContains<T> for RangeInclusive<T> {
    fn fully_contains(&self, other: &RangeInclusive<T>) -> bool {
        let self_len = *self.end() - *self.start();
        let other_len = *other.end() - *other.start();
        let (largest_range, smallest_range) = if self_len > other_len {
            (self, other)
        } else {
            (other, self)
        };

        largest_range.start() <= smallest_range.start()
            && largest_range.end() >= smallest_range.end()
    }
}

pub trait Union<T> {
    fn union(&self, other_range: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
}

impl<T: PartialOrd + Ord + Copy> Union<T> for RangeInclusive<T> {
    fn union(&self, other_range: &RangeInclusive<T>) -> Option<RangeInclusive<T>> {
        if !self.overlaps(other_range) {
            return None;
        }

        Some(*self.start().min(other_range.start())..=*self.end().max(other_range.end()))
    }
}

pub trait GetNumbers<T: FromStr> {
    fn numbers(&self) -> Vec<T>;
    fn number_by_separators(&self, separators: &[char]) -> Vec<T>;
}

impl<T: FromStr> GetNumbers<T> for str {
    fn numbers(&self) -> Vec<T> {
        self.split_whitespace()
            .filter_map(|w| w.parse::<T>().ok())
            .collect_vec()
    }

    fn number_by_separators(&self, separators: &[char]) -> Vec<T> {
        self.split(separators)
            .filter_map(|w| w.parse::<T>().ok())
            .collect_vec()
    }
}
