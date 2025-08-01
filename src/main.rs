use clap::Parser;

use simple_logger::SimpleLogger;
use std::path::PathBuf;

mod samclip;
use samclip::samclip_run;

#[derive(Parser, Debug)]
#[command(
    long_about = "Removes bad alignments from BAM file by considering soft- and hardclipping."
)]
struct Args {
    #[arg(short, long)]
    bam: PathBuf,

    #[arg(long, default_value_t = 10)]
    max_num_softclipped: u32,

    #[arg(long, default_value_t = 10)]
    max_num_hardclipped: u32,

    #[arg(long, default_value_t = 8)]
    threads: usize,

    #[arg(short, long, required = true)]
    outfile: PathBuf,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = Args::parse();

    if args.bam.extension().unwrap() != "bam" {
        panic!("Invalid file extension for provided BAM file.");
    }

    samclip_run(
        &args.bam,
        &args.outfile,
        args.threads,
        args.max_num_softclipped as usize,
        args.max_num_hardclipped as usize,
    );
}
