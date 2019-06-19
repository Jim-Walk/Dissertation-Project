// Port of Babel Stream core functionality
// to Rust
extern crate clap;
extern crate rayon;
extern crate par_array_init;
use clap::{Arg, App};
use std::process::exit;
use rayon::prelude::*;
use std::time::Instant;


fn lin(i: usize, j: usize, shift: i32) -> usize {
    i + (j << shift)
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
                        .parse::<i32>()
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
    rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();

    if lsize < 0 {
        println!("ERROR, log of grid size must be greater than or equal to zero: {}", lsize);
        exit(1);
    }
    
    /* compute number of points in the grid                                         */
    let size2: usize = size * size;

    /* emit error if (periodic) stencil overlaps with itself                        */
    if size < (2 * radius + 1).into() {
        println!("Error, grid extent {} smaller than stencil diameter 2*{}+1={}",
                 size, radius, 2 * radius + 1);
    }

    /* compute total size of star stencil in 2D                                     */
    let stencil_size = 4 * radius as usize + 1;
    
    /* sparsity follows from number of non-zeroes per row                           */
    let sparsity  = (4.0 * radius as f64 + 1.0) / size2 as f64;

    /* compute total number of non-zeroes                                           */
    let nent = size2 as usize * stencil_size;
    println!("Jim's bootleg Research Kernels version 0.17");
    println!("Rust Sparse matrix-vector multiplication");
    println!("Number of threads     =\t{}", num_threads);
    println!("Matrix order          = {}", size2);
    println!("Stencil diameter      = {}", 2 * radius+1);
    println!("Sparsity              = {}", sparsity);
    println!("Number of iterations  = {}", iterations);
    
    let mut result = vec![0.0 as f64, 0.0 as f64];
    let mut vector = vec![0.0 as f64, 0.0 as f64];
    vec![0.0; size2 as usize].par_iter()
        .map(|_| 0.0)
        .collect_into_vec(&mut vector);
    vec![0.0; size2 as usize].par_iter()
        .map(|_| 0.0)
        .collect_into_vec(&mut result);


    // Parallel init uses a trick because we're on a 64bit system
    //(0..size2 as usize).into_par_iter()
    //    .map(|idx| init(idx as i64, size, stencil_size, lsize, lsize2, nent, radius))
    //    .collect_into_vec(&mut matrix);

    let mut matrix = vec![0.0f64; nent];
    let mut colIndex = vec![0usize; nent];

    for row in 0..size2 {
        let j = row / size;
        let i = row % size;
        let mut elm = row * stencil_size;
        colIndex[elm] = lin(i,j,lsize);
        
        for r in 1..=radius{
            colIndex[elm + 1] = lin( (i+r)%size, j, lsize );
            colIndex[elm + 2] = lin( (i-r+size)%size, j, lsize );
            colIndex[elm + 3] = lin( i, (j+r)%size, lsize );
            colIndex[elm + 4] = lin( i, (j-r+size)%size, lsize );
            elm += 4;
        }
        let lo = row as usize * stencil_size;
        let hi = lo + stencil_size;
        &colIndex[lo..hi].sort_unstable_by(|a,b| a.cmp(b));
        let ub = (row + 1) as usize * stencil_size; // upper bound
        for e in lo..ub {
            matrix[e] = 1.0/ (colIndex[e]+1) as f64;
        }
    }
    
    let mut sparse_time = Instant::now();
    for i in 0..=iterations {

        if i == 1 {
            sparse_time = Instant::now();
        }
        // fill vector
        vector.par_iter_mut()
            .enumerate()
            .for_each(|(idx, e)| *e += idx as f64 + 1.0);
        
        for row in 0..size2 as usize{
            let first = stencil_size * row;
            let last = first + stencil_size - 1;
            let mut temp = 0.0;

            for col in first..=last{
                temp += matrix[col] * vector[colIndex[col]];
            }
            result[row] += temp;
        }
    }
    let end_time = sparse_time.elapsed();

    

    // Verisfication test
    let reference_sum = 0.5 * nent as f64 * (iterations + 1) as f64 * (iterations + 2) as f64;

    let mut vector_sum = 0.0;
    for row in 0..size2 as usize { 
        vector_sum += result[row]
    }

    let epsilon = 1.0e-8; // error tolerance
    if (vector_sum-reference_sum).abs() > epsilon {
        println!("ERROR: Vector sum = {}, Reference vector sum = {}", vector_sum, reference_sum);
    } else {
        println!("Solution validates");
    }
    let avgtime = (end_time.as_micros() as f64 /iterations as f64) * 1.0e-6;


    // Print info 
    let rate = 1.0e-6 * (2.0 * nent as f64)/avgtime;
    println!("Rate (MFLops/s = {}, Avg time (s): {}", rate, avgtime);
}
