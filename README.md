# samclip_rs
🚧 Work in progress for re-implementing samclip (https://github.com/tseemann/samclip) by Torsten Seemann.

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the samclip_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/samclip_rs`.

## Usage
Run with:<br>
`samclip_rs --bam <sorted.bam> --outfile <out.bam>`

Optional arguments:
<pre>
<b>--max-num-softclipped</b> [10] - Maximum allowed softclipped bases.

<b>--max-num-hardclipped</b> [10] - Maximum allowed hardclipped bases.

<b>--threads</b> [8] - Threads to use for Rust HTSlib.
</pre>
