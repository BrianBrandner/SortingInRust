use crate::SortingAlg;

pub struct InsertionSort;

impl SortingAlg for InsertionSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        for mut i in 0..array.len() {
            for j in (0..i + 1).rev() {
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
