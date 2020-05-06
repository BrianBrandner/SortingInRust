#[macro_use]
#[warn(unused_imports)]

extern crate stdweb;
extern crate  instant;

use crate::canvas::Canvas;
use std::borrow::{Borrow, BorrowMut};
use bubble_sort::bubble_sort;

mod bubble_sort;
mod random_sort;
mod quick_sort;
mod counting_sort;
mod heap_sort;
mod merge_sort;
mod selection_sort;
mod shell_sort;
mod canvas;

use rand::thread_rng;
use rand::seq::SliceRandom;
use stdweb::web::{set_timeout, document, INonElementParentNode, Element, IParentNode};
use instant::{Instant};


fn main() {
    stdweb::initialize();

    let mut array = create_shuffled_vector(25);
    let max = array.iter().max().unwrap().clone();
    let canvas = Canvas::new("canvas", array.len() as u32, max + 5);
    let mut sorting_steps: Vec<Vec<u32>> = vec![array.clone()];
    let start = Instant::now();
    bubble_sort(array.borrow_mut(), sorting_steps.borrow_mut());
    let duration = start.elapsed().as_micros() as i32;
    let some_element = document().query_selector("#elapsed_time").unwrap().unwrap();

    js!{
    @{some_element}.innerHTML = @{duration};
    };

    draw_step(canvas, sorting_steps);
    stdweb::event_loop();
}

fn draw_step(canvas: Canvas, mut steps:  Vec<Vec<u32>>){
    set_timeout(move ||{
        if !steps.is_empty() {
            let mut remaining_steps = steps.split_off(1);
            draw_array(&canvas, &steps[0]);
            draw_step(canvas, remaining_steps);
        }
    }, 100);
}


fn draw_array(canvas: &Canvas, array: &Vec<u32>) {
    canvas.set_canvase_color("#ccd1d1");
    let mut i: i32 = 0;
    while i < array.len() as i32 {
        draw_column(canvas.borrow(), array[i as usize] as u32, i as u32);
        i = i + 1;
    };
}

fn draw_column(canvas: &Canvas, height: u32, position: u32) {
    let mut i = canvas.height - 1;
    while i >= canvas.height - height {
        canvas.draw(position, i, "green");
        i = i -1;
    }
}

pub fn create_shuffled_vector(length: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = (1..length+1).collect();
    vec.shuffle(&mut thread_rng());
    vec
}


fn create_reversed_vector(length: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = (1..length+1).collect();
    vec.reverse();
    vec
}