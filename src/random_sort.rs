use rand::{thread_rng};
use rand::seq::SliceRandom;

use crate::SortingAlg;

pub struct RandomSort;

/// RandomSort is a meme sorting algorithm, which randomly shuffles list and then checks if it is
/// sorted. The best case scenario is one shuffle, which is pretty fast, but the worst case scenario
/// takes a million times the time of sorting the array manually.
impl RandomSort {
    /// Checks if `array` is sorted.
    fn is_sorted(array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) -> bool {
        let mut previous = array[0];
        for i in 1..array.len() {
            if array[i] < previous {
                return false;
            }
            steps.push(array.clone());
            steps.push(vec![i as u32, (i - 1) as u32]);
            previous = array[i]
        }
        steps.push(array.clone());
        steps.push(vec![]);
        true
    }
}

impl SortingAlg for RandomSort {
    /// Randomly shuffles an array.
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        while !RandomSort::is_sorted(array, steps) {
            array.shuffle(&mut thread_rng());
            steps.push(array.clone());
            steps.push(vec![]);
        }
        steps.push(array.clone());
    }
}
