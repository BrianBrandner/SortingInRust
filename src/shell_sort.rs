pub fn shell_sort(values: &mut [i32]) {
    // shell sort works by swiping the value at a given gap and decreasing the gap to 1

    let mut count_sublist = values.len() / 2; // makes gap as long as half of the array
    while count_sublist > 0 {
        for pos_start in 0..count_sublist {
            insertion(values, pos_start, count_sublist);
        }
        count_sublist /= 2; // makes gap as half of previous
    }
}

//TODO: Idk what .step_by() should be.. :(
fn insertion(values: &mut [i32], start: usize, gap: usize) {
    for i in ((start + gap)..values.len()).step_by(gap) {
        let val_current = values[i];
        let mut pos = i;
        // make swaps
        while pos >= gap && values[pos - gap] > val_current {
            values[pos] = values[pos - gap];
            pos = pos - gap;
        }
        values[pos] = val_current;
    }
}