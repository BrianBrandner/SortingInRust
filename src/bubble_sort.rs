use crate::SortingAlg;

pub struct BubbleSort;

impl SortingAlg for BubbleSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        for i in 0..array.len() {
            for j in 0..array.len() - 1 - i {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                    steps.push(array.clone())
                }
            }
        }
    }
}