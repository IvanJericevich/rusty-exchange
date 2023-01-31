use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stream {
    Fills,
}

impl Stream {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stream::Fills => "fills",
        }
    }

    pub fn iter() -> Iter<'static, Stream> {
        static STREAMS: [Stream; 1] = [Stream::Fills];
        STREAMS.iter()
    }
}
