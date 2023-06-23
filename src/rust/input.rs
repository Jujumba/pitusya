use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
pub struct InputFile {
    content: Vec<char>,
    content_str: String,
    cursor: RefCell<usize>
}

impl InputFile {
    #[inline]
    pub fn current_char(&self) -> char {
        self.content[*self.cursor.borrow()]
    }
    #[inline]
    pub fn out_of_bounds(&self) -> bool {
        *self.cursor.borrow() >= self.content.len()
    }
    #[inline]
    pub fn move_cursor(&self, n: usize) {
        self.cursor.replace(self.get_cursor() + n);
    }
    #[inline]
    pub fn move_back_cursor(&self, n: usize) {
        self.cursor.replace(self.get_cursor() - n);
    }
    #[inline]
    pub fn get_cursor(&self) -> usize {
        *self.cursor.borrow()
    }
    pub fn skip_spaces(&mut self) {
        if self.out_of_bounds() {
            return;
        }
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
            content_str: content,
            cursor: RefCell::new(0)
        }
    }
    pub fn get_str(&self) -> &str {
        &self.content_str
    }
}
