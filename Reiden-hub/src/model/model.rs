use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Spans {
    spans: Vec<Span>,
}

impl Spans {
    pub fn new() -> Self {
        Spans { spans: Vec::new() }
    }

    pub fn add(&mut self, duration: Span) {
        self.spans.push(duration)
    }
}

impl Iterator for Spans {
    type Item = Span;
    fn next(&mut self) -> Option<Self::Item> {
        self.spans.pop()
    }
}





#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Span {
    duration: (String, String),
}

impl Span {
    pub fn new(duration: (String, String)) -> Self {
        Span { duration }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start:{}, end:{}", self.duration.0, self.duration.1)
    }
}
