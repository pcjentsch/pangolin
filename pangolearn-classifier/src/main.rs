mod score;
use crate::score::score;
use csv::*;
use std::fs::File;
use std::process;
use std::time::{Duration,Instant};

fn main() {
    let mut infile = File::open("../../pangolin/temp/encoded_sequences.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(infile);

    let outfile = File::create("output_probabilites.csv").unwrap();

    // let mut wrtr= csv::Writer::from_writer(outfile);

    let now = Instant::now();
    let header_length = rdr.headers().unwrap().len();
    for result in rdr.records(){
        let record = result.unwrap();
        let record_vec: Vec<f64> = record.into_iter().map(|x| x.parse().expect(x)).collect();
        let outputs = score(record_vec);
        println!("{:?}",outputs[1]);
        // wrtr.write_record(output);
    }
    // wrtr.flush().unwrap();
    let timing =now.elapsed();
    println!("took {} ms or {} microseconds",timing.as_millis(), timing.as_micros());

}
//took 479 ms or 479062 microseconds