// Port of Babel Stream core functionality
// to Rust
extern crate clap;
extern crate rayon;
use clap::{Arg, App};
use std::process::exit;
use rayon::prelude::*;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;


fn lin(i: usize, j: usize, shift: i32) -> usize {
    i + (j << shift)
}

// A successful write returns a zero, an unsuccesful returns a 1. 
// This enables us to keep track of the number of unsafe memory accesses attempted
fn safe_write(arr:&mut Vec<usize>, idx: usize, val: usize) -> i32{
    if idx < arr.len() {
        arr[idx] = val;
        0
    } else {
        1
    }
}


fn main() {

    // Configuration Variables declartion
    
    let matches = App::new("Sparse")
                            .version("0.1")
                            .author("Jim Walker j.m.walker@live.co.uk")
                            .arg(Arg::with_name("iterations")
                                 .short("n")
                                 .long("iterations")
                                 .value_name("ITER")
                                 .help("Run the test ITER times")
                                 .takes_value(true))
                            .arg(Arg::with_name("Number of threads")
                                 .short("t")
                                 .long("num_threads")
                                 .value_name("NUM_THREADS")
                                 .help("Use NUM_THREADS")
                                 .takes_value(true))
                            .arg(Arg::with_name("2log grid size")
                                 .short("g")
                                 .long("2log grid size")
                                 .value_name("2LOG")
                                 .help("Use 2LOG to set the logarithmic linear size of the grid")
                                 .takes_value(true))
                            .arg(Arg::with_name("stencil radius")
                                 .short("sr")
                                 .long("stencil radius")
                                 .value_name("STN")
                                 .help("Use STN to set the stencil radius")
                                 .takes_value(true))
                            .get_matches();

    // Set up cmd line variables
    let iterations = matches.value_of("iterations")
                        .unwrap_or("20")
                        .parse::<i32>()
                        .unwrap();
    let num_threads = matches.value_of("Number of threads")
                        .unwrap_or("1")
                        .parse::<usize>()
                        .unwrap();
    let lsize = matches.value_of("2log grid size")
                        .unwrap_or("7")
                        .parse::<i32>()
                        .unwrap();
    let radius = matches.value_of("stencil radius")
                        .unwrap_or("3")
                        .parse::<usize>()
                        .unwrap();
    let size = 1<<lsize;
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    if lsize < 0 {
        println!("ERROR, log of grid size must be greater than or equal to zero: {}", lsize);
        exit(1);
    }
    
    /* compute number of points in the grid                                         */
    let size2: usize = size * size;

    /* emit error if (periodic) stencil overlaps with itself                        */
    if size < (2 * radius + 1) {
        println!("Error, grid extent {} smaller than stencil diameter 2*{}+1={}",
                 size, radius, 2 * radius + 1);
        exit(1);
    }

    /* compute total size of star stencil in 2D                                     */
    let stencil_size = 4 * radius as usize + 1;
    
    /* sparsity follows from number of non-zeroes per row                           */
    let sparsity  = (4.0 * radius as f64 + 1.0) / size2 as f64;

    /* compute total number of non-zeroes                                           */
    let nent = size2 * stencil_size;
    println!("Jim's bootleg Research Kernels version 0.17");
    println!("Rust Sparse matrix-vector multiplication");
    println!("Number of threads     =\t{}", num_threads);
    println!("Matrix order          = {}", size2);
    println!("Stencil diameter      = {}", 2 * radius+1);
    println!("Sparsity              = {}", sparsity);
    println!("Number of iterations  = {}", iterations);
    
    let mut result = vec![0.0 as f64, 0.0 as f64];
    let mut vector = vec![0.0 as f64, 0.0 as f64];
    vec![0.0; size2].par_iter()
        .map(|_| 0.0)
        .collect_into_vec(&mut vector);
    vec![0.0; size2].par_iter()
        .map(|_| 0.0)
        .collect_into_vec(&mut result);



    let mut matrix = vec![0.0f64, 0.0f64];
    let mut col_index = vec![0usize; nent];
    for row in (0..size2).into_iter() {
        let j = row / size;
        let i = row % size;
        let mut elm = row * stencil_size;
        col_index[elm] = lin(i,j,lsize);
        
        for r in 1..=radius{
            col_index[elm + 1] = lin( (i+r)%size, j, lsize );
            col_index[elm + 2] = lin( (i+size-r)%size, j, lsize );
            col_index[elm + 3] = lin( i, (j+r)%size, lsize );
            col_index[elm + 4] = lin( i, (j+size-r)%size, lsize );
            elm += 4;
        }
   }
   // let col_index = Arc::new(Mutex::new(vec![]));

   // // vector of handles
   // let mut hds = vec![];
   // // tx: Transmitter, sends data
   // // rx: Receiver, recieves data
   // let (tx, rx) = channel();

   // for t_id in 0..num_threads {
   //     let (col_index, tx) = (Arc::clone(&col_index), tx.clone());
   //     hds.push(thread::Builder::new().name(t_id.to_string()).spawn(move || {
   //         
   //         // calculate where I am going to write
   //         let interval = size2/num_threads;
   //         let my_lo = t_id * interval;
   //         let mut my_hi = my_lo + interval;
   //         if (t_id + 1) == num_threads{
   //             my_hi = size2;
   //         }
   //         let col_interval = nent/num_threads;
   //         let col_lo = t_id * col_interval;
   //         let mut my_col_index = vec![0usize; col_interval];
   //         let mut misses = 0;
   //         if (t_id + 1) == num_threads {
   //             my_col_index = vec![0usize; nent - col_lo];
   //             //println!("nent: {} col_interval * num_threads: {} total elems: {}", nent, col_interval * num_threads, col_interval * (num_threads-1) + nent - col_lo);
   //             //println!("{}: my_col_index len is {}", t_id, my_col_index.len());
   //         }
   //         for row in my_lo..my_hi {
   //             let j = row / size;
   //             let i = row % size;
   //             let mut elm = (row - my_lo) * stencil_size;
   //             // Write my part of the array
   //             misses += safe_write(&mut my_col_index, elm, lin(i,j,lsize));
   //             for r in 1..=radius{
   //                 misses += safe_write(&mut my_col_index, elm + 1, lin( (i+r)%size, j, lsize ));
   //                 misses += safe_write(&mut my_col_index, elm + 2, lin( (i+size-r)%size, j, lsize ));
   //                 misses += safe_write(&mut my_col_index, elm + 3, lin( i, (j+r)%size, lsize ));
   //                 misses += safe_write(&mut my_col_index, elm + 4, lin( i, (j+size-r)%size, lsize ));
   //                 elm += 4;
   //             }
   //         }
   //         // Aquire lock, check length of array is appropriate for me to add my section
   //         let guard = col_index.lock().unwrap();
   //         let mut len = guard.len();
   //         drop(guard);
   //         // While length is not appropriate, release lock and wait
   //         while len != col_lo{
   //             thread::sleep(time::Duration::from_millis(t_id as u64));
   //             let guard = col_index.lock().unwrap();
   //             len = guard.len();
   //             drop(guard);
   //         }
   //         let mut guard = col_index.lock().unwrap();
   //         guard.append(&mut my_col_index);
   //         if guard.len() == nent {
   //             tx.send(()).unwrap();
   //         }
   //         if misses > 0 {
   //             println!("{}: attempted to write out of bounds {} times", t_id, misses);
   //         }
   //     }));
   // }
   // rx.recv().unwrap();
   // for h in hds {
   //     h.unwrap().join().unwrap()
   // }
   // let col_index = col_index.lock().unwrap();

    col_index.par_iter().map(|e| 1.0/(e+1) as f64).collect_into_vec(&mut matrix);
    
    let mut sparse_time = Instant::now();
    for i in 0..=iterations {

        if i == 1 {
            sparse_time = Instant::now();
        }
        // fill vector
        vector.par_iter_mut()
            .enumerate()
            .for_each(|(idx, e)| *e += idx as f64 + 1.0);
        
        result.par_iter_mut().enumerate().for_each( |(row, item)| {
            let first = stencil_size * row;
            let last = first + stencil_size - 1;
            let mut temp = 0.0;

            // Consider adding explicit SIMD here
            for col in first..=last{
                temp += matrix[col] * vector[col_index[col]];
            }
            *item += temp;
        });
    }
    let end_time = sparse_time.elapsed();

    // Verification test
    let reference_sum = 0.5 * nent as f64 * f64::from(iterations + 1) * f64::from(iterations + 2);

    let vector_sum: f64 = result.iter().sum();

    let epsilon = 1.0e-8; // error tolerance
    if (vector_sum-reference_sum).abs() > epsilon {
        println!("ERROR: Vector sum = {}, Reference vector sum = {}", vector_sum, reference_sum);
    } else {
        println!("Solution validates");
    }
    let avgtime = (end_time.as_micros() as f64 /iterations as f64) * 1.0e-6;

    // Print info 
    let rate = 1.0e-6 * (2.0 * nent as f64)/avgtime;
    println!("Rate (MFLops/s): {}, Avg time (s): {}", rate, avgtime);
}
