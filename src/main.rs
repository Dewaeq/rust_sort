#![feature(is_sorted)]
#![feature(bench_black_box)]

use std::hint::black_box;
use std::thread;
use std::time::Instant;

use rand::prelude::SliceRandom;
use rust_sort::prelude::*;
use rust_sort::rand_array;

const NUM_TESTS: usize = 10;
const ARRAY_SIZE: usize = 30_000;

fn main() {
    let mut handles = vec![];

    handles.push(thread::spawn(|| {
        bench(selection, "selection");
    }));
    handles.push(thread::spawn(|| {
        bench(bubble, "bubble");
    }));
    handles.push(thread::spawn(|| {
        bench(insertion, "insertion");
    }));
    handles.push(thread::spawn(|| {
        bench(gnome, "gnome");
    }));
    handles.push(thread::spawn(|| {
        bench(quick, "quick");
    }));
    handles.push(thread::spawn(|| {
        bench(merge, "Merge");
    }));

    for h in handles {
        h.join().unwrap();
    }
}

fn bench(function: fn(&mut [i32]), name: &str) {
    let time = Instant::now();
    let ar = &mut rand_array::<ARRAY_SIZE>();
    let mut rng = rand::thread_rng();

    for _ in 0..NUM_TESTS {
        ar.shuffle(&mut rng);
        function(black_box(ar));
    }

    println!(
        "{} took {:>width$}ms",
        name,
        time.elapsed().as_millis(),
        width = 15 - name.len()
    );
}
