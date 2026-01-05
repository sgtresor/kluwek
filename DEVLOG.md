# Kluwek DevLog

## 2026-01-05: Project Initialization & Performance Baseline
**Goal:** Establish a high-performance Rust environment for DNA processing.
**Dataset:** E. coli (SRR12669768), ~2.7M reads.

### Benchmarks
- **Task:** Count reads in a gzipped FASTQ file.
- **Implementation:** Rust `BufReader` + `flate2` (GzDecoder).
- **Result:** 2.76M reads in 2.39s (~1.15M reads/sec).
- **Hardware:** Ubuntu PC (Local).

### Key Learnings
- FASTQ files have a 4-line structure.
- Rust's `?` operator is critical for handling IO Results cleanly.
- `BufReader` is mandatory; raw file reading is too slow for genomic data.

### Next Steps
- Parse the actual DNA sequences (Lines 2 and 4).
- Calculate "GC Content" (a basic biological metric).