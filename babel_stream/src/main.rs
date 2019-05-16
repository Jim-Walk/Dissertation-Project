// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};
use std::time::Instant;
mod stream;

//use crate::stream;

fn main() {

    // Configuration Variables declartion
    // Default size is 2^25
    
    let matches = App::new("babel_stream")
                            .version("0.1")
                            .author("Jim Walker j.m.walker@live.co.uk")
                            .arg(Arg::with_name("float")
                                 .short("f")
                                 .long("float")
                                 .help("Use floats (rather than doubles)")
                                 .takes_value(false))
                            .arg(Arg::with_name("numtimes")
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
    let num_times = matches.value_of("numtimes")
                        .unwrap_or("100")
                        .parse::<i32>()
                        .unwrap();
    let use_float = matches.is_present("float");
    let array_size = matches.value_of("arraysize")
                        .unwrap_or("33554432")
                        .parse::<f32>()
                        .unwrap();

    // Set up initial variables
    let start_a = 0.1f32;
    let start_b = 0.2f32;
    let start_c = 0.0f32;
    let sscalar = 0.4f32;
    let mut sum = 0.0f32;
    // Print info 
    println!("BabelStream");
    println!("Version: 0.1");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", num_times);
    if use_float {
        println!("Precision: float");
        println!("Array size: {:.1} MB (={:.1} GB)",
                    array_size*4.0*1.0E-6,
                    array_size*4.0*1.0E-9);
        println!("Total size: {:.1} MB (={:.1} GB)",
                    3.0*array_size*4.0*1.0E-6,
                    3.0*array_size*4.0*1.0E-9);
    } else {
        println!("Precision: double");
        println!("Array size: {:.1} MB (={:.1} GB)",
                    array_size*8.0*1.0E-6,
                    array_size*8.0*1.0E-9);

        println!("Total size: {:.1} MB (={:.1} GB)",
                    3.0*array_size*8.0*1.0E-6,
                    3.0*array_size*8.0*1.0E-9);
        let array_size = array_size as f64;
        let start_a = start_a as f64;
        let start_b = start_b as f64;
        let start_c = start_c as f64;
        let sscalar = sscalar as f64;
        let sum = sum as f64;
    }



    let mut stream = stream::RustStream {
        a: vec![start_a, array_size],
        b: vec![start_b, array_size],
        c: vec![start_c, array_size],
        scalar: sscalar,
    };

    // List of times
    let mut timings: [Vec<u128>; 5] = Default::default();

    for _i in 0..num_times{
            // Execute copy
            let t1 = Instant::now();
            stream.copy();
            let t2 = t1.elapsed();
            timings[0].push(t2.as_micros());
            
            // Execute mul
            let t1 = Instant::now();
            stream.mul();
            let t2 = t1.elapsed();
            timings[1].push(t2.as_micros());

            // Execute add
            let t1 = Instant::now();
            stream.add();
            let t2 = t1.elapsed();
            timings[2].push(t2.as_micros());

            // Execute triad
            let t1 = Instant::now();
            stream.triad();
            let t2 = t1.elapsed();
            timings[3].push(t2.as_micros());

            // Execute dot
            let t1 = Instant::now();
            sum = stream.dot();
            let t2 = t1.elapsed();
            timings[4].push(t2.as_micros());
    }

    // Check results
    let start_vals = [start_a, start_b, start_c];
    stream.check_solution(num_times, start_vals, array_size, sscalar, sum);

    // Print timings
    
    let labels = vec!["Copy", "Mul", "Add", "Triad", "Dot"];
    println!("Function\tMbytes/sec\tMin (sec)\tMax\tAverage");
    for i in 0..5{
        let label = labels[i];
        let Mbs = 0;
        let min = timings[0].iter().min().unwrap();
        let max = timings[0].iter().max().unwrap();
        println!("{}\t{}\t{}\t{}\t???", label, Mbs, min, max)
    }

}
