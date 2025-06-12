use samclip_rs::alignments::{alignment_type, valid_clip};
use samclip_rs::types::AlignmentType;

#[test]
fn test_valid_clip() {
    assert_eq!(true, valid_clip(&10, 10));
    assert_eq!(false, valid_clip(&15, 10));
}

#[test]
fn test_alignment_types() {
    assert_eq!(AlignmentType::Start, alignment_type(0, 100, 200));
    assert_eq!(AlignmentType::End, alignment_type(50, 200, 200));
    assert_eq!(AlignmentType::Middle, alignment_type(50, 150, 200));
    assert_eq!(AlignmentType::Full, alignment_type(0, 200, 200));
}
