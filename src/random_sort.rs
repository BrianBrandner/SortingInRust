use rand::{thread_rng};
use rand::seq::SliceRandom;

use crate::SortingAlg;

pub struct RandomSort;

/// RandomSort is a meme sorting algorithm. It randomly shuffles the vector and checks if it's now
/// sorted. The best case scenario is faster than any other sorting algorithm, but it's worst case
/// is slower than any other sorting algorithm. Please do not use this with vector sizes bigger than
/// 5. Thank you.
impl SortingAlg for RandomSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        steps.clear();
        while !is_sorted(array,steps) {
            array.shuffle(&mut thread_rng());
            steps.push(array.clone());
            steps.push(vec![]);
        }
        steps.push(array.clone());
    }
}

/// This function is called every time after the vector has been randomly shuffled to check if it's
/// now sorted.
fn is_sorted(array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) -> bool {
    let mut previous = array[0];
    for i in 1..array.len() {
        if array[i] < previous {
            return false
        }
        steps.push(array.clone());
        steps.push(vec![i as u32, (i -1) as u32]);
        previous = array[i]
    }
    steps.push(array.clone());
    steps.push(vec![]);
    return true
}