#[derive(Debug, PartialEq)]
pub enum AlignmentType {
    Start,
    End,
    Middle,
    Full,
}

#[derive(Debug)]
pub struct Reference {
    pub name: String,
    pub len: usize,
}
