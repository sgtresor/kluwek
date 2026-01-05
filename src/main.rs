use std::fs::File;
use std::io::{self, BufRead, BufReader};
use flate2::read::GzDecoder;
use std::time::Instant;

fn main() -> io::Result<()> {
    let path = "./data/SRR12669768_1.fastq.gz";
    println!("Processing file: {}", path);

    let start = Instant::now();

    // 2. Open the file 
    // The '?' here is crucial - it un-wraps the file or crashes safely if not found
    let file = File::open(path)?; 

    // 3. Attach the Gunzip decoder
    let decoder = GzDecoder::new(file);
    
    // 4. Wrap in a Buffer (Speed)
    let reader = BufReader::new(decoder);

    let mut line_count = 0;
    
    // 5. Iterate through lines
    for _ in reader.lines() {
        line_count += 1;
    }

    let duration = start.elapsed();
    let record_count = line_count / 4;
    
    println!("--------------------------------");
    println!("Total Reads: {}", record_count);
    println!("Time taken:  {:.2?}", duration);
    println!("--------------------------------");

    Ok(())
}