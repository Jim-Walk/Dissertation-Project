// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};
use std::time::Instant;
mod stream;

fn main() {

    // Configuration Variables declartion
    // Default size is 2^25
    
    let matches = App::new("int_stream")
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
                        .parse::<usize>()
                        .unwrap();

    // Set up initial variables
    // Print info 
    println!("Int Stream");
    println!("Version: 0.1");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", num_times);
    let print_size = array_size as f32;
    let start_a = 1i32;
    let start_b = 2i32;
    let start_c = 0i32;
    let sscalar = 4i32;
    println!("Precision: i32");
    println!("Array size: {:.1} MB (={:.1} GB)",
                print_size*4.0*1.0E-6,
                print_size*4.0*1.0E-9);
    println!("Total size: {:.1} MB (={:.1} GB)",
                3.0*print_size*4.0*1.0E-6,
                3.0*print_size*4.0*1.0E-9);
    // We fully init the array later
    let mut stream = stream::RustStream {
        a: vec![start_a; 4],
        b: vec![start_b; 4],
        c: vec![start_c; 4],
        scalar: sscalar,
    };
    let start_vals = [start_a, start_b, start_c];
    run(stream, start_vals, print_size, num_times);
}

pub fn run(mut my_stream: stream::RustStream, start_vals: [i32;3], array_size: f32, num_times: i32) 
{

    my_stream.init_arrays(array_size as i32);
    // List of times
    let mut timings: [Vec<u128>; 5] = Default::default();

    let mut sum = 0;
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

    // Check results
    my_stream.check_solution(num_times, start_vals, array_size, sum);

    // Print timings
    let labels = vec!["Copy", "Mul", "Add", "Triad", "Dot"];
    println!("Function\tMbytes/sec\tMin (sec)\tMax\t\tAverage");

    let mem_size = std::mem::size_of::<i32>() as f64;

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
