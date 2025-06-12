use rust_htslib::bam::Record;
use rust_htslib::bam::record::Cigar;

use crate::types::AlignmentType;

#[inline]
pub fn is_primary(record: &Record) -> bool {
    return !record.is_unmapped() && !record.is_secondary() && !record.is_supplementary();
}

#[inline]
pub fn alignment_type(aln_start: i64, aln_end: i64, ref_len: usize) -> AlignmentType {
    if aln_start == 0 && (aln_end as usize) < ref_len {
        return AlignmentType::Start;
    };

    if aln_start > 0 && (aln_end as usize) == ref_len {
        return AlignmentType::End;
    };

    if aln_start > 0 && (aln_end as usize) < ref_len {
        return AlignmentType::Middle;
    };

    if aln_start == 0 && (aln_end as usize) == ref_len {
        return AlignmentType::Full;
    };

    panic!("Alignment has unexpected coordinates.");
}

#[inline]
pub fn valid_clip(clip: &u32, max_clip: usize) -> bool {
    return (*clip as usize) <= max_clip;
}

/// If the start of the contig is aligned, we need to check clipping in the end of the alignment.
/// contig      -----------------------
/// read    ssss-----------????
#[inline]
pub fn valid_start_alignment(
    right: &Cigar,
    max_num_softclipped: usize,
    max_num_hardclipped: usize,
) -> bool {
    match right {
        Cigar::SoftClip(clip) => {
            return valid_clip(clip, max_num_softclipped);
        }
        Cigar::HardClip(clip) => {
            return valid_clip(clip, max_num_hardclipped);
        }
        _ => return true,
    }
}

/// If the end of the contig is aligned, we need to check clipping in the start of the alignment.
/// contig      -----------------------
/// read                ????-----------ssss
#[inline]
pub fn valid_end_alignment(
    left: &Cigar,
    max_num_softclipped: usize,
    max_num_hardclipped: usize,
) -> bool {
    match left {
        Cigar::SoftClip(clip) => {
            return valid_clip(clip, max_num_softclipped);
        }
        Cigar::HardClip(clip) => {
            return valid_clip(clip, max_num_hardclipped);
        }
        _ => return true,
    }
}

/// If the middle of the contig is aligned, we need to check clipping in the ends of the alignment.
/// contig      -----------------------
/// read            ????------????
#[inline]
pub fn valid_middle_alignment(
    left: &Cigar,
    right: &Cigar,
    max_num_softclipped: usize,
    max_num_hardclipped: usize,
) -> bool {
    let valid_left = match left {
        Cigar::SoftClip(clip) => valid_clip(clip, max_num_softclipped),
        Cigar::HardClip(clip) => valid_clip(clip, max_num_hardclipped),
        _ => true,
    };

    let valid_right = match right {
        Cigar::SoftClip(clip) => valid_clip(clip, max_num_softclipped),
        Cigar::HardClip(clip) => valid_clip(clip, max_num_hardclipped),
        _ => true,
    };

    return valid_left && valid_right;
}
