#[macro_use]
#[warn(unused_imports)]

extern crate stdweb;
extern crate  instant;

use crate::canvas::Canvas;
use std::borrow::{Borrow, BorrowMut};
use bubble_sort::BubbleSort;
use shell_sort::ShellSort;
use selection_sort::SelectionSort;
use quick_sort::QuickSort;
use merge_sort::MergeSort;
use random_sort::RandomSort;

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
use stdweb::web::{set_timeout, document, INonElementParentNode, Element, IParentNode, IEventTarget, IElement, INode};
use instant::{Instant};
use stdweb::web::event::{ClickEvent, IEvent, InputEvent};
use std::rc::Rc;
use std::sync::{RwLock, Mutex};
use crate::heap_sort::HeapSort;
use crate::counting_sort::CountingSort;

trait SortingAlg {
    fn sort(&self,array: & mut Vec<u32>, steps: &mut Vec<Vec<u32>>);
}

struct SortingProcess {
    canvas: Canvas,
    delay: Rc<RwLock<u32>>,
    length: u32,
    sorting_steps: Vec<Vec<u32>>,
    abort: bool,
}

fn main() {
    stdweb::initialize();
    let slider_time = document().query_selector( "#millsec" ).unwrap().unwrap();
    let mut delay: Rc<RwLock<u32>> = {
        let attr_value = slider_time.get_attribute("value").unwrap();
        Rc::new(  RwLock::new(attr_value.parse().expect("not an Number")))
    };
    let temp_rc = delay.clone();
    slider_time.clone().add_event_listener( move |current_delay: InputEvent| {
        let attr_value = slider_time.get_attribute("value").unwrap();
        *temp_rc.write().unwrap() = attr_value.parse().expect("not an Number");
    });

    let mut current_process: Option<Rc<RwLock<SortingProcess>>> = None;

    let button = document().query_selector( "#start" ).unwrap().unwrap();
    button.add_event_listener( move | sorting: ClickEvent| {
        if let Some(process) = &current_process {
            process.write().unwrap().abort = true;
        }
        let length :u32 = document().get_element_by_id("size").unwrap().get_attribute("value").unwrap().parse().expect("not an Number");
        let value_of_selected = js!{
                    var e = document.getElementById("sorting");
                    return e.options[e.selectedIndex].value;
                    };
        let sorting_selection = value_of_selected.as_str().unwrap();
        current_process = match sorting_selection {
            "bubble" => Some(start_sorting(length,delay.clone(), &BubbleSort)),
            "quick" =>  Some(start_sorting(length,delay.clone(), &QuickSort)),
            "counting" => Some(start_sorting(length,delay.clone(), &CountingSort)),
            "heap" => Some(start_sorting(length,delay.clone(), &HeapSort)),
            "shell" =>  Some(start_sorting(length,delay.clone(), &ShellSort)),
            "merge" => Some(start_sorting(length,delay.clone(), &MergeSort)),
            "random" =>  Some(start_sorting(length,delay.clone(), &RandomSort)),
            "selection" => Some(start_sorting(length,delay.clone(), &SelectionSort)),
            _ => panic!("not Implemented"),
        };
    });
    stdweb::event_loop();
}

fn start_sorting<S: SortingAlg>(length: u32, delay : Rc<RwLock<u32>>, sorting_alg: &S) -> Rc<RwLock<SortingProcess>> {
    let generating_value = js!{
                    var e = document.getElementById("generating");
                    return e.options[e.selectedIndex].value;
                    };
    let generating_alg = generating_value.as_str().unwrap();
    let mut array = match generating_alg {
        "random" => create_shuffled_vector(length),
        "reversed" => create_reversed_vector(length),
        "n-2" => create_n_2_vector(length),
        _ => panic!("not Implemented"),
    };
    let canvas = Canvas::new("canvas", length, length + 5);
    let mut sorting_steps: Vec<Vec<u32>> = vec![vec![]];
    let start = Instant::now();
    sorting_alg.sort(&mut array, &mut sorting_steps);
    let duration = start.elapsed().as_millis() as i32;
    let time_element = document().query_selector("#elapsed_time").unwrap().unwrap();
    js!{
    @{time_element}.innerHTML = @{duration};
    };
    let time_element = document().query_selector("#steps").unwrap().unwrap();
    let steps_needed = ((sorting_steps.len()-1)/4) as i32;
    js!{
    @{time_element}.innerHTML = @{steps_needed};
    };
    let sorting_process = Rc::new(RwLock::new(SortingProcess{
        canvas,
        delay,
        length,sorting_steps,
        abort: false,
    }));
    draw_step(sorting_process.clone());
    sorting_process
}

fn draw_step(sorting_process: Rc<RwLock<SortingProcess>>){
    if sorting_process.read().unwrap().abort {
        sorting_process.read().unwrap().canvas.set_canvase_color("white");
        return;
    }
    let delay = *sorting_process.read().unwrap().delay.read().unwrap();
    set_timeout(move ||{
        let mut process = sorting_process.write().unwrap();
        if process.sorting_steps.len() >= 2{
            js!(console.log("Ahllo"+ @{&process.sorting_steps}););
            let remaining_steps = process.sorting_steps.split_off(2);
            draw_array(&process.canvas, &process.sorting_steps[0], &process.sorting_steps[1]);
            process.sorting_steps = remaining_steps;
            drop(process);
            draw_step(sorting_process);
        }
    }, delay);
}


fn draw_array(canvas: &Canvas, array: &Vec<u32>, color: &Vec<u32>) {
    canvas.set_canvase_color("white");
    let mut i: i32 = 0;
    while i < array.len() as i32 {
        let temp: u32 = i as u32;
        let color = if color.contains(&temp){
            "red"
        }else {
            "green"
        };
        draw_column(canvas.borrow(), array[i as usize] as u32, i as u32, color);
        i = i + 1;
    };
}

fn draw_column(canvas: &Canvas, height: u32, position: u32, color: &str) {
    let mut i = canvas.height - 1;
    while i >= canvas.height - height {
        canvas.draw(position, i, color);
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

fn create_n_2_vector(length: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    vec.push(1);

    for i in 1..length - 1 {
        vec.push(length / 2);
    }
    vec.push(length);

    vec.shuffle(&mut thread_rng());
    vec
}