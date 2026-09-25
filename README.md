# encoding-algs

Two compression algorithms implemented in Rust: run-length encoding (RLE) and Huffman coding. I built them to understand how compression works.

## Algorithms

### Run-length encoding (`rle-encoding/`)

Repeated bytes are saved as pairs of *(count, byte)*. For example, `aaab` becomes `[3, 'a', 1, 'b']`. If a byte repeats more than 255 times, it is split into several pairs. Encoding and decoding both work.

### Huffman coding (`huffman-encoding/`)

Builds a Huffman tree based on how often each byte appears. Bytes that appear often get shorter codes, rare bytes get longer ones. Encoding works, decoding is not done yet.

## Run

```sh
cd rle-encoding
cargo run

cd ../huffman-encoding
cargo run
```

Both programs encode a short example text and print the result.

## What I learned
- How lossless compression works in principle
- How to build a binary tree in Rust

