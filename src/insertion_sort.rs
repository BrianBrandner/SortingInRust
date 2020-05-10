use crate::SortingAlg;

pub struct InsertionSort;

/// InsertionSort is an in place O(n²) sorting algorithm that moves an element until it hits a
/// smaller element. Starting with the first element, which is already sorted, this procedure takes
/// exponentially more time as the sorted part grows.
impl SortingAlg for InsertionSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        for mut i in 0..array.len() {
            for j in (0..i).rev() {
                steps.push(array.clone());
                steps.push(vec![i as u32, j as u32]);

                if array[i] < array[j] {
                    array.swap(i, j);
                    steps.push(array.clone());
                    steps.push(vec![i as u32, j as u32]);
                    i -= 1;
                } else {
                    // If a smaller element is hit, a new iteration of the outer loop will begin.
                    break;
                }
            }
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}
