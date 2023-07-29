use std::cell::RefCell;
use std::fs;
use std::io;

#[derive(Debug, PartialEq, Clone)]
pub struct InputFile {
    content: Vec<char>,
    content_str: String,
    cursor: RefCell<usize>,
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
    pub fn new(file_name: &str) -> io::Result<Self> {
        let content = fs::read_to_string(file_name)?;
        Ok(Self {
            content: content.chars().collect(),
            content_str: content,
            cursor: RefCell::new(0)
        })
    }
}
impl AsRef<str> for InputFile {
    fn as_ref(&self) -> &str {
        &self.content_str
    }
}
