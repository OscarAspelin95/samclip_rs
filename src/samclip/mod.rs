pub mod samclip;
pub use samclip::samclip_run;

pub mod alignments;
pub use alignments::{
    alignment_type, is_primary, valid_clip, valid_end_alignment, valid_middle_alignment,
    valid_start_alignment,
};

pub mod types;
pub use types::{AlignmentType, Reference};
