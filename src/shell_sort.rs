use crate::SortingAlg;

pub struct ShellSort;

impl SortingAlg for ShellSort {
    fn sort(&self, array: &mut Vec<u32>, steps: &mut Vec<Vec<u32>>) {
        // shell sort works by swiping the value at a given gap and decreasing the gap to 1

        let mut count_sublist = array.len() / 2; // makes gap as long as half of the array
        while count_sublist > 0 {
            for pos_start in 0..count_sublist {
                insertion(array, pos_start, count_sublist, steps);
            }
            count_sublist /= 2; // makes gap as half of previous
        }
    }
}

//TODO: Idk what .step_by() should be.. :(
fn insertion(array: &mut Vec<u32>, start: usize, gap: usize, steps: &mut Vec<Vec<u32>>) {
    for i in ((start + gap)..array.len()).step_by(gap) {
        let val_current = array[i];
        let mut pos = i;
        // make swaps
        while pos >= gap && array[pos - gap] > val_current {
            array[pos] = array[pos - gap];
            pos = pos - gap;
            steps.push(array.clone())
        }
        array[pos] = val_current;
    }
}