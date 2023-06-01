pub struct InputFile {
    pub content: Vec<char>,
    pub cursor: usize,
}

impl InputFile {
    pub fn skip_spaces(&mut self) {
        while self.content[self.cursor].is_whitespace() {
            self.cursor += 1;
        }
    }
    #[inline]
    pub fn current_char(&self) -> char {
        self.content[self.cursor]
    }
    #[inline]
    pub fn get_substr_to_cursor(&self, start: usize) -> String {
        if start == self.content.len() {
            String::from(self.content[start])
        } else {
            String::from_iter(&self.content[start..self.cursor])
        }
    }
    #[inline]
    pub fn out_of_bounds(&self) -> bool {
        self.cursor >= self.content.len()
    }
}