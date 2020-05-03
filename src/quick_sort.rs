fn _partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
            println!("1{:?}", arr);
        }
    }
    arr.swap(i as usize, pivot as usize);
    println!("2{:?}", arr);
    i
}

fn _quick_sort(arr: &mut [i32], lo: isize, hi: isize) {
    if lo < hi {
        let p = _partition(arr, lo, hi);
        println!("3{:?}", arr);
        _quick_sort(arr, lo, p - 1);
        println!("4{:?}", arr);
        _quick_sort(arr, p + 1, hi);
        println!("5{:?}", arr);
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
    println!("6{:?}", arr);
}