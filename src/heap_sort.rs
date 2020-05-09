/// Sort a mutable slice using heap sort.
///
/// Heap sort is an in-place O(n log n) sorting algorithm. It is based on a
/// max heap, a binary tree data structure whose main feature is that
/// parent nodes are always greater or equal to their child nodes.

use crate::SortingAlg;

pub struct HeapSort;

impl SortingAlg for HeapSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        steps.clear();

        if array.len() <= 1 {
            return; // already sorted
        }

        heapify(array, steps);

        for end in (1..array.len()).rev() {
            array.swap(0, end);
            steps.push(array.clone());
            steps.push(vec![0 as u32, end as u32]);

            let mut temp = array.split_off(end);
            move_down(array, 0, steps);

            array.append(&mut temp)
        }
        steps.push(array.clone());
        steps.push(vec![]);
    }
}

/// Convert `array` into a max heap.
fn heapify(array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
    let last_parent = (array.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(array, i, steps);
        steps.push(array.clone());
        steps.push(vec![0 as u32, 0 as u32]);
    }
}

/// Move the element at `root` down until `arr` is a max heap again.
///
/// This assumes that the subtrees under `root` are valid max heaps already.
fn move_down(array: &mut Vec<u32>, mut root: usize, steps: &mut Vec<Vec<u32>>) {
    let last = array.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && array[right] > array[left] {
            right
        } else {
            left
        };

        if array[max] > array[root] {
            array.swap(root, max);
        }
        root = max;
    }
}