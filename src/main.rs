mod bubble_sort;
mod random_sort;
mod quick_sort;
mod counting_sort;
mod heap_sort;
mod merge_sort;
mod selection_sort;
mod shell_sort;


use rand::thread_rng;
use rand::seq::SliceRandom;


fn main() {
    println!("Hello, world!");
}


fn create_vector(length: i32) {
    let mut vec: Vec<i32> = (0..length).collect();
    vec.shuffle(&mut thread_rng())
}