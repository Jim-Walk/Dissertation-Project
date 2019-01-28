extern crate rand;

use std::env;
use rand::Rng;

fn main() {
    let args: Vec<_> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    
    let mut x: Vec<f32> = Vec::with_capacity(size);
    let mut y: Vec<f32> = Vec::with_capacity(size);
    let mut rng = rand::thread_rng();
    
    for i in 0..size{
        x.push(rng.gen::<f32>());
        y.push(rng.gen::<f32>());
    }

    println!("Running saxpy on arrays of size {}", size);

    let f = 2.0;

    saxpy(size, f, x, y);
}

fn saxpy(n: usize, a: f32, x: Vec<f32>, mut y: Vec<f32>){
    

    for i in 0..n {
        y[i] = a*x[i] + y[i];
    }

}
