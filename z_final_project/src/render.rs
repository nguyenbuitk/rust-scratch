// crossterm lib để hiển thị các khung hình (frames) trên một terminal trong Rust.
use std::io::Stdout;
use std::io::Write;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;

// Render what is changed, Nếu force là true, thì sẽ vẽ tất cả.
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // render everything, clear everything to blue
    if force {
        // Cho các command vào queue và thực hiện 1 lượt
        // unwrap() is crash when there is error
        // chúng ta đặt màu nền của terminal thành màu xanh 
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();

        // sau đó xóa tất cả nội dung trên terminal
        stdout.queue(Clear(ClearType::All)).unwrap();

        // và cuối cùng, đặt màu nền trở lại màu đen 
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // nếu force = 1 hoặc ký tự so với frame trước đó thay đổi thì sẽ vẽ lại
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    // đảm bảo rằng tất cả các lệnh trong hàng đợi được thực hiện và dữ liệu được hiển thị trên màn hình
    stdout.flush().unwrap();
}