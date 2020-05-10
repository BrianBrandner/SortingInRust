/// In place counting sort
/// O(n + maxval) in time, where maxval is the biggest value an input can possibly take
/// O(maxval) in memory
/// u32 is chosen arbitrarly, a counting sort probably shouldn't be used on data that requires bigger types.

use crate::SortingAlg;

pub struct CountingSort;

/// Linear, integer only sorting algorithm, best case O(n*log(n)). Sorts them according to the keys.
///
impl SortingAlg for CountingSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let mut occurences: Vec<usize> = vec![0; array.len() + 1];

        for &data in array.iter() {
            occurences[data as usize] += 1;
        }

        let mut i = 0;
        for (data, &number) in occurences.iter().enumerate() {
            for _ in 0..number {
                steps.push(array.clone());
                steps.push(vec![i as u32]);
                array[i] = data as u32;
                steps.push(array.clone());
                steps.push(vec![i as u32]);
                i += 1;
            }
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}
