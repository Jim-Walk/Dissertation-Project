// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};

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
                            .get_matches();

    // Set up cmd line variables
    let iterations = matches.value_of("iterations")
                        .unwrap_or("20")
                        .parse::<i32>()
                        .unwrap();
    let num_threads = matches.value_of("Number of threads")
                        .unwrap_or("20")
                        .parse::<i32>()
                        .unwrap();

    // Set up initial variables
    // Print info 
    println!("Parallel Research Kernels version 2.17");
    println!("Rust Sparse matrix-vector multiplication");
    println!("Number of threads\t=\t{}", num_threads);
    println!("Number of iterations\t=\t {}", iterations);
}
