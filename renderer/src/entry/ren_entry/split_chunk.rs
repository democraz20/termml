use core::iter;

pub trait CharChunksTrait<'a> {
    fn char_chunks(self, n: usize) -> CharChunks<'a>;
}

impl<'a> CharChunksTrait<'a> for &'a str {
    fn char_chunks(self, n: usize) -> CharChunks<'a> {
        CharChunks::new(self, n)
    }
}

//unicode segmentation crate
//for unicode grapheme support (https://crates.io/crates/unicode-segmentation)
// struct GraphemeChunks<'a> {
//     str: &'a str,
//     iter: core::iter::Peekable<core::iter::StepBy<unicode_segmentation::GraphemeIndices<'a>>>,
// }

pub struct CharChunks<'a> {
    str: &'a str,
    iter: iter::Peekable<iter::StepBy<core::str::CharIndices<'a>>>,
}

impl<'a> CharChunks<'a> {
    fn new(str: &'a str, n: usize) -> Self {
        CharChunks {
            str,
            iter: str.char_indices().step_by(n).peekable(),
        }
    }
}

impl<'a> Iterator for CharChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let (start, _) = self.iter.next()?;
        let end = self
            .iter
            .peek()
            .map(|&(end, _)| end)
            .unwrap_or(self.str.len());

        Some(&self.str[start..end])
    }
}
