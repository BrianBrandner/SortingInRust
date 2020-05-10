use crate::SortingAlg;

pub struct MergeSort;

impl MergeSort {
    fn merge(array: &mut Vec<u32>, low: usize, middle: usize, high: usize, steps: &mut Vec<Vec<u32>>) {
        // create temporary arrays to support merge
        let mut left_half = Vec::new();
        let mut right_half = Vec::new();
        for i in low..middle + 1 {
            left_half.push(array[i]);
        }
        for i in middle + 1..high + 1 {
            right_half.push(array[i]);
        }

        let lsize = left_half.len();
        let rsize = right_half.len();

        // pointers to track the positions while merging
        let mut left = 0;
        let mut right = 0;
        let mut temp = low;

        // pick smaller element one by one from either left or right half
        while left < lsize && right < rsize {
            if left_half[left] < right_half[right] {
                steps.push(array.clone());
                steps.push(vec![temp as u32, left as u32]);
                array[temp] = left_half[left];
                steps.push(array.clone());
                steps.push(vec![temp as u32, left as u32]);
                left += 1;
            } else {
                steps.push(array.clone());
                steps.push(vec![temp as u32, right as u32]);
                array[temp] = right_half[right];
                steps.push(array.clone());
                steps.push(vec![temp as u32, right as u32]);
                right += 1;
            }
            temp += 1;
        }
        // put all the remaining ones
        while left < lsize {
            steps.push(array.clone());
            steps.push(vec![temp as u32, left as u32]);
            array[temp] = left_half[left];
            steps.push(array.clone());
            steps.push(vec![temp as u32, left as u32]);
            left += 1;
            temp += 1;
        }

        while right < rsize {
            steps.push(array.clone());
            steps.push(vec![temp as u32, right as u32]);
            array[temp] = right_half[right];
            steps.push(array.clone());
            steps.push(vec![temp as u32, right as u32]);
            right += 1;
            temp += 1;
        }
    }

    fn merge_sort(array: &mut Vec<u32>, low: usize, high: usize, steps: &mut Vec<Vec<u32>>) {
        if low < high {
            let middle = low + (high - low) / 2;
            MergeSort::merge_sort(array, low, middle, steps);
            MergeSort::merge_sort(array, middle + 1, high, steps);
            MergeSort::merge(array, low, middle, high, steps);
        }
    }
}

impl SortingAlg for MergeSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let length = array.len();
        if length > 1 {
            MergeSort::merge_sort(array, 0, length - 1, steps);
        }
        steps.push(array.clone());
        steps.push(vec![])
    }
}
