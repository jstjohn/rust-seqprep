
extern crate clap;
extern crate bio;

use clap::{Arg, App};
use findadapters::findadapters::example;
mod findadapters;

//SubCommand};

fn main() {
    let matches = App::new("SeqPrep2")
        .version("2.0")
        .author("John St. John <john@driver.xyz>")
        .about("Remove pesky barcodes from your FR paired end illumina reads")
        .arg(Arg::with_name("inr1")
             .short("f")
             .long("inr1")
             .value_name("FILE")
             .help("Read 1 input fastq file")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("inr2")
             .short("r")
             .long("inr2")
             .value_name("FILE")
             .help("Read 2 input fastq file")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("outr1")
             .short("1")
             .long("outr1")
             .value_name("FILE")
             .help("Read 1 trimmed output fastq file")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("outr2")
             .short("2")
             .long("outr2")
             .value_name("FILE")
             .help("Read 2 trimmed output fastq file")
             .takes_value(true)
             .required(true))
        .get_matches();
    println!("Running on input files {} {}", 
             matches.value_of("inr1").unwrap(),
             matches.value_of("inr2").unwrap());
    example(matches.value_of("inr1").unwrap().as_bytes());
    println!("Hello, world!");
}
