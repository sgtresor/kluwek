use std::fs::File;
use std::io::{self, BufRead, BufReader};
use flate2::read::{GzDecoder};
use std::time::{Instant};

fn main() -> io::Result<()> {
    let path = "./data/SRR12669768_1.fastq.gz";
    println!("Processing file: {}", path);

    let start = Instant::now();
    let file = File::open(path)?; 
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let mut total_reads: usize = 0;
    let mut total_bases: usize = 0;
    let mut gc_count: usize = 0;

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;

        // Logic: FASTQ format repeats every 4 lines.
        // Index 0 = Header
        // Index 1 = Sequence (This is what we want!)
        // Index 2 = Separator
        // Index 3 = Quality
        if index % 4 == 1 {
            total_reads += 1;
            // Iterate over raw bytes (u8) instead of chars to avoid UTF-8 overhead.
            // b'G' is the byte representation of the character G.
            for byte in line.bytes() {
                // incrementing total_bases for every valid letter
                total_bases += 1;
                match byte {
                    b'G' | b'C' => gc_count +=1, // count G or C
                    b'N' => {
                        // rare edge case: 'N' means the machine couldn't read the letter.
                        // usually we dont count 'N' in the total for strict GC calculation,
                        // but for simplicity, we count it as a base, just not a G or C
                    },
                    _ => {} // do nothing for A or T
                }
            }
        }
    }

    let duration = start.elapsed();

    let gc_percentage = if total_bases > 0 {
        (gc_count as f64 / total_bases as f64) * 100.0
    } else {
        0.0
    };
    
    println!("--------------------------------");
    println!("Analysis Complete.");
    println!("Total Reads Processed: {}", total_reads);
    println!("Total DNA Bases:       {}", total_bases);
    println!("GC Count:              {}", gc_count);
    println!("GC Content:            {:.2}%", gc_percentage);
    println!("Time taken:            {:.2?}", duration);
    println!("--------------------------------");

    Ok(())
}