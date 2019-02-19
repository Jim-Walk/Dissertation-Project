extern crate rand;
use std::time::Instant;
use rand::prelude::*;
use std::env;

fn main() {
    //let size = 100000;
    let args: Vec<_> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    let mut x: Vec<f32> = Vec::with_capacity(size);
    let mut y: Vec<f32> = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        // 1 (inclusive) to 21 (exclusive)
        x.push(rng.gen_range(1.0, 51.0));
        y.push(rng.gen_range(1.0, 21.0));
    }

    let index = rng.gen_range(1, 21);
    println!("y[{}] is {}", index, y[index]);
    println!("Running saxpy on arrays of size {}", size);

    let f = 2.0;
    let mut i = 0;
    let mut done = false;
    let start = Instant::now();
    while !done {
        saxpy(&f, &x, &mut y);
        i += i+1;
        if i >= 100000{
            done = true;
        }
        
    }
    let end = Instant::now();
    println!("Done! Executed in {:?}", end.duration_since(start));
    println!("y[{}] is {}", index, y[index]);
}

fn saxpy(a: &f32, x: &Vec<f32>, y: &mut Vec<f32>){
  
    for (y_i, x_i) in y.iter_mut().zip(x){
        *y_i += a*x_i;
    }
}
/*    for i in 0..n {
        y[i] = a*x[i] + y[i];
    }
*/

