use crate::SortingAlg;

pub struct InsertionSort;

/// InsertionSort is an in place O(n²) sorting algorithm that moves an element until it hits a
/// smaller element. Starting with the first element, which is already sorted, this procedure takes
/// exponentially more time as the sorted part grows.
impl SortingAlg for InsertionSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        steps.clear();
        for mut i in 0..array.len() {
            for j in (0..i+1).rev() {
                if array[i] < array[j] {
                    steps.push(array.clone());
                    steps.push(vec![i as u32, j as u32]);
                    array.swap(i, j);
                    steps.push(array.clone());
                    steps.push(vec![i as u32, j as u32]);
                    i -= 1;
                }
            }
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}