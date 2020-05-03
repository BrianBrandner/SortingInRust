fn _merge(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    // create temporary arrays to support merge
    let mut left_half = Vec::new();
    let mut right_half = Vec::new();
    for i in lo..mid + 1 {
        left_half.push(arr[i]);
    }
    for i in mid + 1..hi + 1 {
        right_half.push(arr[i]);
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
            arr[a] = left_half[l];
            l += 1;
        } else {
            arr[a] = right_half[r];
            r += 1;
        }
        a += 1;
    }

    // put all the remaining ones
    while l < lsize {
        arr[a] = left_half[l];
        l += 1;
        a += 1;
    }

    while r < rsize {
        arr[a] = right_half[r];
        r += 1;
        a += 1;
    }
}

fn _merge_sort(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < hi {
        let mid = lo + (hi - lo) / 2;
        _merge_sort(arr, lo, mid);
        _merge_sort(arr, mid + 1, hi);
        _merge(arr, lo, mid, hi);
    }
}

pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        _merge_sort(arr, 0, len - 1);
    }
}
