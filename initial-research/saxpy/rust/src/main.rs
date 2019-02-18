use std::time::Instant;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    
    let mut x: Vec<f32> = Vec::with_capacity(size);
    let mut y: Vec<f32> = Vec::with_capacity(size);

  for i in 0..size{
      y.push(6.0);
      x.push(9.0);
  }

    println!("Running saxpy on arrays of size {}", size);

    let f = 2.0;
    let mut i = 0;
    let mut done = false;
    let start = Instant::now();
    while !done {
        saxpy(f, x, y);
        i += i+1;
        if i >= 100{
            done = true;
        }
        
    }
    let end = Instant::now();
    println!("Done! Executed in {:?}", end.duration_since(start));
}

fn saxpy(a: f32, x: Vec<f32>, mut y: Vec<f32>){
  
    for (y_i, x_i) in y.iter_mut().zip(x){
        *y_i += a*x_i;
    }
}
/*    for i in 0..n {
        y[i] = a*x[i] + y[i];
    }
*/

