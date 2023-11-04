// cấu trúc dữ liệu Frame, một hàm new_frame, và một trait Drawable
use crate::{NUM_COLS, NUM_ROWS};

// định nghĩa một kiểu dữ liệu mới trong Rust, được đặt tên là Frame, mà là một vector hai chiều (2D vector) chứa các chuỗi mượn (&str) với lifetime 'static
// khi khai báo chuỗi mượn (&str) thì bắt buộc phải khai báo lifetime
pub type Frame = Vec<Vec<& 'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col: Vec<&str> = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}