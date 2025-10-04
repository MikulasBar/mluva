

pub struct Path {
    segments: Vec<String>,   
}

impl Path {
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn single(segment: String) -> Self {
        Self { segments: vec![segment] }
    }
}