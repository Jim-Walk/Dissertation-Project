// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};
use std::time::Instant;
mod stream;
//use num::traits;
use std::any::Any;
use std::ops::{AddAssign, DivAssign};
use num::Float;

fn main() {

    // Configuration Variables declartion
    // Default size is 2^25
    
    let matches = App::new("babel_stream")
                            .version("0.5")
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
                        .parse::<usize>()
                        .unwrap();

    // Set up initial variables
    // Print info 
    println!("BabelStream");
    println!("Version: 0.5");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", num_times);
    if use_float {
        let print_size = array_size as f32;
        println!("Precision: float");
        println!("Array size: {:.1} MB (={:.1} GB)",
                    print_size*4.0*1.0E-6,
                    print_size*4.0*1.0E-9);
        println!("Total size: {:.1} MB (={:.1} GB)",
                    3.0*print_size*4.0*1.0E-6,
                    3.0*print_size*4.0*1.0E-9);
        // We will properly initalise these arrays in parallel
        // in main
        let mut stream = stream::RustStream {
            a: vec![0.1f32; 2],
            b: vec![0.2f32; 2],
            c: vec![0.0f32; 2],
            scalar: 0.4f32,
            chunk_size: 1875000
        };
        let start_vals = [0.1f32, 0.2f32, 0.0f32];
        run(stream, start_vals, array_size, num_times);
    } else {
        let print_size = array_size as f64;
        println!("Precision: double");
        println!("Array size: {:.1} MB (={:.1} GB)",
                    print_size*8.0*1.0E-6,
                    print_size*8.0*1.0E-9);

        println!("Total size: {:.1} MB (={:.1} GB)",
                    3.0*print_size*8.0*1.0E-6,
                    3.0*print_size*8.0*1.0E-9);
        // We will properly initalise these arrays in parallel
        // in main
        let mut stream = stream::RustStream {
            a: vec![0.1f64; 2],
            b: vec![0.2f64; 2],
            c: vec![0.0f64; 2],
            scalar: 0.4f64,
            chunk_size: 1875000
        };
        let start_vals = [0.1f64, 0.2f64, 0.0f64];
        run(stream, start_vals, array_size, num_times);
    }
}

pub fn run<T>(mut my_stream: stream::RustStream<T>, start_vals: [T;3], array_size: usize, num_times: i32) 
where T: Float + AddAssign<T> + num::Signed + DivAssign<T> + std::fmt::Display + Send + Sync + std::iter::Sum + Any,
[T]: Send + Sync, f64: std::convert::From<T>

{

    my_stream.init_arrays(array_size);
    // List of times
    let mut timings: [Vec<u128>; 5] = Default::default();

    let mut sum: T = T::from(0).unwrap();
    for _i in 0..num_times{
            // Execute copy
            let t1 = Instant::now();
            my_stream.copy();
            let t2 = t1.elapsed();
            timings[0].push(t2.as_micros());
            
            // Execute mul
            let t1 = Instant::now();
            my_stream.mul();
            let t2 = t1.elapsed();
            timings[1].push(t2.as_micros());

            // Execute add
            let t1 = Instant::now();
            my_stream.add();
            let t2 = t1.elapsed();
            timings[2].push(t2.as_micros());

            // Execute triad
            let t1 = Instant::now();
            my_stream.triad();
            let t2 = t1.elapsed();
            timings[3].push(t2.as_micros());

            // Execute dot
            let t1 = Instant::now();
            sum = my_stream.dot();
            let t2 = t1.elapsed();
            timings[4].push(t2.as_micros());
    }
    let array_size = T::from(array_size).unwrap();
    // Check results
    my_stream.check_solution(num_times, start_vals, array_size, sum);

    // Print timings
    let labels = vec!["Copy", "Mul", "Add", "Triad", "Dot"];
    println!("Function\tMbytes/sec\tMin (sec)\tMax\t\tAverage");

    let mem_size = std::mem::size_of::<T>() as f64;

    let size_a = 2.0 * mem_size * my_stream.a.len() as f64;
    let size_b = 3.0 * mem_size * my_stream.a.len() as f64;

    let sizes = [size_a, size_a, size_b, size_b, size_a];

    for i in 0..5{
        let label = labels[i];
        let min = *timings[i].iter().min().unwrap() as f64 / 1.0E6;
        let max = *timings[i].iter().max().unwrap() as f64 / 1.0E6; 
        let m_bs = 1.0E-6 * (sizes[i] as f64) / min;
        let mut avg = timings[i].iter().fold(0u128, | acc, val | acc + val) as f64;
        avg /= timings[i].len() as f64; 
        avg /= 1.0E6;
        println!("{}\t\t{:.3}\t{:.5}\t\t{:.5}\t\t{:.5}", label, m_bs, min, max, avg)
    }
 }
