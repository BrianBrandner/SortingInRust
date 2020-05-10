use crate::SortingAlg;

pub struct BubbleSort;

/// Bubble sort moves the largest element to the end of the vector.

impl SortingAlg for BubbleSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        for i in 0..array.len() {
            for j in 0..array.len() - 1 - i {
                steps.push(array.clone());
                steps.push(vec![j as u32, (j + 1) as u32]);
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                    steps.push(array.clone());
                    steps.push(vec![j as u32, (j + 1) as u32]);
                }
            }
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}
