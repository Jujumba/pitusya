use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

pub use clap::{Parser, Subcommand};

use crate::abort;

#[derive(Parser)]
#[command(
    author = "Jujumba",
    version,
    about = "
The Pitusya Programming Language (=^ ◡ ^=)
"
)]
pub struct Cli {
    file: PathBuf,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CursoredFile {
    pub(crate) name: PathBuf,
    pub(crate) content: Vec<char>,
    pub(crate) content_str: String,
    pub(crate) cursor: RefCell<usize>,
}
impl From<Cli> for PathBuf {
    fn from(value: Cli) -> Self {
        value.file
    }
}
impl CursoredFile {
    pub fn new<P: Into<PathBuf>>(file_name: P) -> Self {
        let file_name = file_name.into();
        let content = match fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(_) => abort!("File {} does not exist!", file_name.display()),
        };
        Self {
            name: file_name,
            content: content.chars().collect(),
            content_str: content,
            cursor: RefCell::new(0),
        }
    }
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
}
impl AsRef<str> for CursoredFile {
    fn as_ref(&self) -> &str {
        &self.content_str
    }
}
impl AsRef<[char]> for CursoredFile {
    fn as_ref(&self) -> &[char] {
        &self.content
    }
}
