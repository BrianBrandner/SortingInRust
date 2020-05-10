use crate::SortingAlg;

pub struct StalinSort;

/// StalinSort is a meme sorting algorithm which neutralizes every element which is not in order.
impl SortingAlg for StalinSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let mut i = 0;
        while i < array.len() - 1 {
            steps.push(array.clone());
            steps.push(vec![i as u32, (i+1) as u32]);
            if array[i] > array[i+1] {
                steps.push(array.clone());
                steps.push(vec![(i+1) as u32]);
                array.remove(i+1);
                steps.push(array.clone());
                steps.push(vec![]);
            } else {
                i += 1;
            }
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}