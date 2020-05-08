use crate::SortingAlg;

pub struct MergeSort;

fn _merge(array: &mut Vec<u32>, lo: usize, mid: usize, hi: usize, steps: &mut Vec<Vec<u32>>) {
    // create temporary arrays to support merge
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
            array[a] = left_half[l];
            l += 1;
        } else {
            array[a] = right_half[r];
            r += 1;
        }
        a += 1;
    }

    // put all the remaining ones
    while l < lsize {
        array[a] = left_half[l];
        l += 1;
        a += 1;
    }

    while r < rsize {
        array[a] = right_half[r];
        r += 1;
        a += 1;
    }
    steps.push(array.clone())
}

fn _merge_sort(array: &mut Vec<u32>, lo: usize, hi: usize, steps: &mut Vec<Vec<u32>>) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _merge_sort(array, lo, mid, steps);
        _merge_sort(array, mid + 1, hi, steps);
        _merge(array, lo, mid, hi, steps);
    }
}

impl SortingAlg for MergeSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let len = array.len();
        if len > 1 {
            _merge_sort(array, 0, len - 1, steps);
        }
    }
}
