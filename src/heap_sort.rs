/// Sort a mutable slice using heap sort.
///
/// Heap sort is an in-place O(n log n) sorting algorithm. It is based on a
/// max heap, a binary tree data structure whose main feature is that
/// parent nodes are always greater or equal to their child nodes.

pub fn heap_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return; // already sorted
    }

    heapify(arr);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        move_down(&mut arr[..end], 0);
    }
}

/// Convert `arr` into a max heap.
fn heapify(arr: &mut [i32]) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i);
    }
}

/// Move the element at `root` down until `arr` is a max heap again.
///
/// This assumes that the subtrees under `root` are valid max heaps already.
fn move_down(arr: &mut [i32], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
        }
        root = max;
    }
}