pub type FileId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub file: FileId,
    pub lo: usize,
    pub hi: usize,
}

impl Span {
    pub fn new(file: FileId, lo: usize, hi: usize) -> Self {
        Self { file, lo, hi }
    }

    pub fn join(self, other: Span) -> Span {
        assert_eq!(self.file, other.file);

        Span {
            file: self.file,
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }
}
