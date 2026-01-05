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

## 2026-01-05: Phase 2 - Sequence Profiling & Optimization
**Goal:** Parse DNA sequences to calculate GC Content.

### Implementation Details
- **Logic:** Iterated over Line 2 of every FASTQ record.
- **Optimization:** Switched from `chars()` (UTF-8) to `bytes()` (u8/ASCII) to minimize CPU overhead.
- **Results:**
    - Total Bases: ~259,490,948
    - GC Content: 49.12% (Matches expected *E. coli* profile).

### The Rust vs. Python Benchmark
We compared this Rust implementation against an identical Python script.
- **Rust:** 3.21s
- **Python:** 21.95s
- **Verdict:** Rust provided a ~7x speedup due to zero-cost abstractions, strict memory layout (u8 vs PyObject), and lack of interpreter overhead.

### Next Steps
- Implement Quality Control (QC) metrics.
- specific focus on parsing Phred Quality Scores (Line 4).