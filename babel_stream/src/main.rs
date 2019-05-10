// Port of Babel Stream core functionality
// to Rust
extern crate clap;
use clap::{Arg, App};

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

//    let ARRAY_SIZE = args.0;
    let num_times = matches.value_of("numtimes").unwrap_or("100").parse::<i32>().unwrap();
    let use_float = matches.is_present("float");
//    let triad_only = args.3;
    println!("BabelStream");
    println!("Version: 0.1");
    println!("Implmentation: Rust");
    println!("Running kernels {} times", num_times);
    if (use_float){
        println!("Precision: single");
    } else {
        println!("Precision: double");
    }
    println!("Array size: ???");
    println!("Total size: ???");
}

