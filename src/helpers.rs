/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

use std::collections::HashSet;

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
