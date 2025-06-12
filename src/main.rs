use clap::Parser;

use simple_logger::SimpleLogger;
use std::path::PathBuf;

use samclip_rs::bam_parse::bam_parse;

#[derive(Parser, Debug)]
#[command(
    long_about = "Removes bad alignments from BAM file by considering soft- and hardclipping."
)]
struct CommandArgs {
    #[arg(short, long)]
    bam: PathBuf,

    #[arg(long, default_value = "10", value_parser= clap::value_parser!(u32).range(1..100_000))]
    max_num_softclipped: u32,

    #[arg(long, default_value = "10", value_parser= clap::value_parser!(u32).range(1..100_000))]
    max_num_hardclipped: u32,

    #[arg(long, default_value = "1", value_parser = clap::value_parser!(u32).range(1..))]
    threads: u32,

    #[arg(short, long, required = true)]
    outfile: PathBuf,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = CommandArgs::parse();

    if args.bam.extension().unwrap() != "bam" {
        panic!("Invalid file extension for provided BAM file.");
    }

    bam_parse(
        &args.bam,
        &args.outfile,
        args.threads as usize,
        args.max_num_softclipped as usize,
        args.max_num_hardclipped as usize,
    );
}
