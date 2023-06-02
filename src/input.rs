pub struct InputFile {
    pub content: Vec<char>,
    pub cursor: usize,
}

impl InputFile {
    #[inline]
    pub fn current_char(&self) -> char {
        self.content[self.cursor]
    }
    #[inline]
    pub fn out_of_bounds(&self) -> bool {
        self.cursor >= self.content.len()
    }
    #[inline]
    pub fn move_cursora(&mut self, n: usize) {
        self.cursor += n
    }
}