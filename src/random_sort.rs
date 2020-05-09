use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::SortingAlg;

pub struct RandomSort;

impl SortingAlg for RandomSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        while !is_sorted(array) {
            steps.push(array.clone());
            array.shuffle(&mut thread_rng());
        }
        steps.push(array.clone());
    }
}

fn is_sorted(array: &mut Vec<u32>) -> bool {
    let mut previous = array[0];
    for i in 1..array.len() {
        if array[i] < previous {
            return false
        }
        previous = array[i]
    }
    return true
}