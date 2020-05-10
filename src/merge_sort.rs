use crate::SortingAlg;

pub struct MergeSort;


fn _merge(array: &mut Vec<u32>, lo: usize, mid: usize, hi: usize, steps: &mut Vec<Vec<u32>>) {
    // create temporary vectors to support merge
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();
    for i in lo..mid + 1 {
        left_half.push(array[i]);
    }
    for i in mid + 1..hi + 1 {
        right_half.push(array[i]);
    }

        let lsize = left_half.len();
        let rsize = right_half.len();

        // pointers to track the positions while merging
        let mut l = 0;
        let mut r = 0;
        let mut a = lo;

        // pick smaller element one by one from either left or right half
        while l < lsize && r < rsize {
            if left_half[l] < right_half[r] {
                steps.push(array.clone());
                steps.push(vec![a as u32, l as u32]);
                array[a] = left_half[l];
                steps.push(array.clone());
                steps.push(vec![a as u32, l as u32]);
                l += 1;
            } else {
                steps.push(array.clone());
                steps.push(vec![a as u32, r as u32]);
                array[a] = right_half[r];
                steps.push(array.clone());
                steps.push(vec![a as u32, r as u32]);
                r += 1;
            }
            a += 1;
        }
        // put all the remaining ones
        while l < lsize {
            steps.push(array.clone());
            steps.push(vec![a as u32, l as u32]);
            array[a] = left_half[l];
            steps.push(array.clone());
            steps.push(vec![a as u32, l as u32]);
            l += 1;
            a += 1;
        }

        while r < rsize {
            steps.push(array.clone());
            steps.push(vec![a as u32, r as u32]);
            array[a] = right_half[r];
            steps.push(array.clone());
            steps.push(vec![a as u32, r as u32]);
            r += 1;
            a += 1;
        }
    }

    fn merge_sort(array: &mut Vec<u32>, lo: usize, hi: usize, steps: &mut Vec<Vec<u32>>) {
        if lo < hi {
            let mid = lo + (hi - lo) / 2;
            MergeSort::merge_sort(array, lo, mid, steps);
            MergeSort::merge_sort(array, mid + 1, hi, steps);
            MergeSort::merge(array, lo, mid, hi, steps);
        }
    }
}

impl SortingAlg for MergeSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let len = array.len();
        if len > 1 {
            MergeSort::merge_sort(array, 0, len - 1, steps);
        }
        steps.push(array.clone());
        steps.push(vec![])
    }
}
