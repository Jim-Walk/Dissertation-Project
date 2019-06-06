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
                                 .long("numtimes")
                                 .value_name("NUM")
                                 .help("Run the test NUM times")
                                 .takes_value(true))
                            .arg(Arg::with_name("arraysize")
                                 .short("s")
                                 .long("arraysize")
                                 .value_name("SIZE")
                                 .help("Use SIZE elements in the array")
                                 .takes_value(true))
                            .get_matches();

    // Set up cmd line variables
    let iterations = matches.value_of("iterations")
                        .unwrap_or("20")
                        .parse::<i32>()
                        .unwrap();
    let use_float = matches.is_present("float");
    let array_size = matches.value_of("arraysize")
                        .unwrap_or("33554432")
                        .parse::<usize>()
                        .unwrap();

    // Set up initial variables
    // Print info 
    println!("Sparse");
    println!("Version: 0.1");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", iterations);
}
