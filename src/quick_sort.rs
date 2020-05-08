use crate::SortingAlg;

pub struct QuickSort;

fn _partition(array: &mut Vec<u32>, lo: isize, hi: isize, steps: &mut Vec<Vec<u32>>) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while array[i as usize] < array[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && array[j as usize] > array[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            array.swap(i as usize, j as usize);
        }
    }
    array.swap(i as usize, pivot as usize);
    steps.push(array.clone());
    i
}

fn _quick_sort(array: &mut Vec<u32>, lo: isize, hi: isize, steps: &mut Vec<Vec<u32>>) {
    if lo < hi {
        let p = _partition(array, lo, hi, steps);
        _quick_sort(array, lo, p - 1, steps);
        _quick_sort(array, p + 1, hi, steps);
    }
}

impl SortingAlg for QuickSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let len = array.len();
        _quick_sort(array, 0, (len - 1) as isize, steps);
    }
}
