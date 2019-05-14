// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};
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
                                 .value_name("SIZE")
                                 .help("Use SIZE elements in the array")
                                 .takes_value(true))
                            .get_matches();

    let num_times = matches.value_of("numtimes").unwrap_or("100").parse::<i32>().unwrap();
    let use_float = matches.is_present("float");
    let array_size = 10;
    println!("BabelStream");
    println!("Version: 0.1");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", num_times);
    println!("Precision: single");
    let mut stream_data = stream::RustStream {
        a: vec![0.1, 10.0],
        b: vec![0.2, 10.0],
        c: vec![0.0, 10.0]
    };
    println!("Array size: ???");
    println!("Total size: ???");

    stream_data.triad();

}

