use crate::SortingAlg;

pub struct SelectionSort;

impl SortingAlg for SelectionSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        steps.clear();
        let len = array.len();
        for left in 0..len {
            let mut smallest = left;
            for right in (left + 1)..len {
                if array[right] < array[smallest] {
                    smallest = right;
                }
            }
            steps.push(array.clone());
            steps.push(vec![left as u32, smallest as u32]);
            array.swap(smallest, left);
            steps.push(array.clone());
            steps.push(vec![left as u32, smallest as u32]);
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}
