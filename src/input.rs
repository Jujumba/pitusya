#[derive(Debug, PartialEq, Clone)]
pub struct InputFile {
    content: Vec<char>,
    cursor: usize,
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
    #[inline]
    pub fn move_back_cursor(&mut self, n: usize) {
        self.cursor -= n;
    }
    #[inline]
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }
    pub fn skip_spaces(&mut self) {
        while !self.out_of_bounds() && self.current_char().is_whitespace() {
            self.move_cursor(1);
        }
    }
    pub fn read_to_string(&self) -> String {
        let mut s = String::new();
        s.reserve(self.content.len());
        self.content.iter().for_each(|c| s.push(*c));
        s
    }
    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect(),
            cursor: 0,
        }
    }
}
