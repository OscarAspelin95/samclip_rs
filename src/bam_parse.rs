use log::info;
use rust_htslib::bam::Header;
use rust_htslib::bam::ext::BamRecordExtensions;
use rust_htslib::bam::record::Cigar;
use rust_htslib::{bam, bam::Read, bam::Reader, bam::Writer};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::alignments::{
    alignment_type, is_primary, valid_end_alignment, valid_middle_alignment, valid_start_alignment,
};

use crate::types::{AlignmentType, Reference};

/// From the BAM header, extract reference information (names and lengths).
/// We do this to avoid having to read from a provided .fasta or .fasta.fai file.
fn get_reference_hmap(bam: &Reader) -> HashMap<usize, Reference> {
    let header_map = bam::Header::from_template(bam.header()).to_hashmap();

    // We extract the SQ field, which is an array in the form of [{"SN": "ref1", "LN": "ref1_len"}, ...].
    // and convert it to a hashmap like: {0: {"name": "ref1", "len": ref1_len}, ...}.
    let reference_hmap: HashMap<usize, Reference> = header_map
        .get("SQ")
        .expect("Missing SQ field in BAM header, cannot extract reference information.")
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let name = entry
                .get("SN")
                .expect("Missing SN field in BAM header, failed to extract reference information.");
            let len = entry
                .get("LN")
                .expect("Missing LN field in BAM header, failed to extract reference information.")
                .parse::<usize>()
                .unwrap();

            let ref_struct = Reference {
                name: name.clone(),
                len: len,
            };

            return (i, ref_struct);
        })
        .collect();

    return reference_hmap;
}

pub fn bam_parse(
    bam: &PathBuf,
    outfile: &PathBuf,
    threads: usize,
    max_num_softclipped: usize,
    max_num_hardclipped: usize,
) {
    let mut bam_reader: Reader =
        bam::Reader::from_path(&bam).expect("Failed to read BAM file {bam}.");
    bam_reader.set_threads(threads).unwrap();

    // Hashmap of reference names and lengths.
    let reference_hmap = get_reference_hmap(&bam_reader);

    // We extract the original bam header and use it in the new bam file.
    let header = Header::from_template(&bam_reader.header());
    let mut bam_out = Writer::from_path(outfile, &header, bam::Format::Bam).unwrap();
    bam_out.set_threads(threads).unwrap();

    let mut records = bam_reader.records();
    let mut num_invalid: usize = 0;

    info!("Parsing records...");
    while let Some(Ok(record)) = records.next() {
        // We only consider primary alignments.
        if !is_primary(&record) {
            continue;
        }

        // Also, do a quick check for any clipping at all in alignment.
        let cigar_stats = record.cigar_stats_nucleotides();

        match cigar_stats.get(&Cigar::SoftClip(0)) {
            None => {
                // Record is valid (no clip), write to file.
                bam_out.write(&record).unwrap();
                continue;
            }
            Some(clip) => {
                // Record is valid (few clips), write to file.
                if (*clip as usize) <= max_num_softclipped {
                    bam_out.write(&record).unwrap();
                    continue;
                }
                // Record might be invalid. Need to continue parsing.
                else {
                    {}
                }
            }
        };

        // Extract the current reference from the alignment.
        let reference_info = reference_hmap.get(&(record.tid() as usize)).unwrap();
        let reference_len = reference_info.len;

        // Coordinates of the alignment.
        let aln_start = record.reference_start();
        let aln_end = record.reference_end();

        // Clipping happens in the start/end of the alignment. We extract the first
        // and last cigar values since we need to look for soft/hardclipping here.
        let cigar = record.cigar();
        let left = cigar.first().unwrap();
        let right = cigar.last().unwrap();

        let aln_type = alignment_type(aln_start, aln_end, reference_len);

        let valid_alignment: bool = match aln_type {
            AlignmentType::Start => {
                valid_start_alignment(&right, max_num_softclipped, max_num_hardclipped)
            }
            AlignmentType::Middle => {
                valid_middle_alignment(&left, &right, max_num_softclipped, max_num_hardclipped)
            }
            AlignmentType::End => {
                valid_end_alignment(&left, max_num_softclipped, max_num_hardclipped)
            }
            // If read spans the entire reference, we don't need to check clipping.
            AlignmentType::Full => true,
        };

        match valid_alignment {
            true => bam_out.write(&record).unwrap(),
            false => num_invalid += 1,
        }
    }

    info!("Removed {} alignments.", num_invalid);
}
