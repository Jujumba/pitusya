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
    pub fn move_cursor(&mut self, n: usize) {
        self.cursor += n
    }
    pub fn skip_spaces(&mut self) {
        while !self.out_of_bounds() && self.current_char().is_whitespace() {
            self.move_cursor(1);
        }
    }
}