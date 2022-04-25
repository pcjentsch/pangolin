mod score;
mod keep_indices;
use crate::score::score;
use csv;
use std::fs::File;
use std::process;
use std::time::{Duration,Instant};
use seq_io::fasta::{self, Record};
use core::fmt::Error;

fn alignment_to_one_hot(fname: String) -> Result<Vec<u8>,Error>{
    let mut fasta_reader = fasta::Reader::from_path(fname).expect("alignment file cannot be opened!");
    while let Some(record) = fasta_reader.next(){
        let record = record.expect("error in reading record from alignment");
        let seq = record.full_seq();
    }
    return Ok(vec![0_u8,0_u8]);
}


fn main() {

    // alignment_to_one_hot("src/test_data/big_test_alignment.fasta".to_string()).unwrap();
    let mut infile = File::open("src/test_data/encoded_sequences.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(infile);

    let outfile = File::create("src/test_data/output_probabilites.csv").unwrap();
    let header_length = rdr.headers().unwrap().len();
    let records: Vec<Vec<f32>> = rdr.records().map(|result|{
        let record = result.unwrap();
        let record_slice = record.as_slice();
        let parsed: Vec<f32> = record_slice[1..].chars()
            .into_iter().map(|x| x.to_digit(10).expect(&x.to_string()) as f32).collect();
        return parsed
    }).collect();
    let now = Instant::now();
    for i in 1..100{
        let outputs: Vec<Vec<f32>> = records.iter().map(|x| score(x.to_vec())).collect();
        println!("{:?}",outputs[1][1]);
    }
    // }
    let timing =now.elapsed();
    println!("took {} ms or {} microseconds",timing.as_millis()/100, timing.as_micros()/100);
}
//took 479 ms or 479062 microseconds
// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
//     fn test_one_hot(){

//     }
        
// }