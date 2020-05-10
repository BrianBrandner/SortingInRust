use crate::SortingAlg;

pub struct QuickSort;

/// QuickSort is a quick in-place O(n*log(n)) sorting algorithm but in its worst case it can hit
/// O(n²)
impl QuickSort {
    fn partition(array: &mut Vec<u32>, low: isize, high: isize, steps: &mut Vec<Vec<u32>>) -> isize {
        let pivot = high as usize;
        let mut i = low - 1;
        let mut j = high;

        loop {
            steps.push(array.clone());
            steps.push(vec![i as u32, pivot as u32]);
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
                steps.push(array.clone());
                steps.push(vec![i as u32, j as u32]);
                array.swap(i as usize, j as usize);
                steps.push(array.clone());
                steps.push(vec![i as u32, j as u32]);
            }
        }
        array.swap(i as usize, pivot as usize);
        steps.push(array.clone());
        steps.push(vec![i as u32, pivot as u32]);
        i
    }

    fn quick_sort(array: &mut Vec<u32>, low: isize, high: isize, steps: &mut Vec<Vec<u32>>) {
        if low < high {
            let p = QuickSort::partition(array, low, high, steps);
            QuickSort::quick_sort(array, low, p - 1, steps);
            QuickSort::quick_sort(array, p + 1, high, steps);
        }
    }
}

impl SortingAlg for QuickSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        let len = array.len();
        QuickSort::quick_sort(array, 0, (len - 1) as isize, steps);
        steps.push(array.clone());
        steps.push(vec![]);
    }
}
