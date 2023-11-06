#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use std::error::Error;
use std::thread;
// use rusty_audio::Audio;
use crossterm::{event, execute, terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use z_final_project::player::Player;
use std::time::{Duration, Instant};
use std::io;
use std::sync::mpsc;
use z_final_project::frame;
use z_final_project::render;
use z_final_project::frame::new_frame;
use z_final_project::frame::Drawable;

fn main() -> Result <(), Box<dyn Error>> {
    // let mut audio = Audio::new();
    // audio.add("explode", "explode.wav");
    // audio.add("lose", "lose.wav");
    // audio.add("move", "move.wav");
    // audio.add("pew", "pew.wav");
    // audio.add("startup", "startup.wav");
    // audio.add("win", "win.wav");
    // audio.play("pew");

    // Terminal
    // get access to stdout
    let mut stdout = io::stdout();

    // enable raw mode for get keyboard input as it occurs.
    // `?` character meaning it will essentially crash if we have error
    terminal::enable_raw_mode()?;

    // execute extension for intermediately execute something
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in seperate thread
    let (render_tx, render_rx) = mpsc::channel();
    
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        // Per frame init, khởi tạo một khung hình (frame) mới cho mỗi lần lặp của game loop.
        let mut curr_frame = new_frame();

        // Input handling, kiểm tra xem có sự kiện đầu vào nào từ người chơi không và trả về true nếu có. Nếu không có sự kiện nào, vòng lặp kết thúc
        while event::poll(Duration::default())? {
            // : Kiểm tra sự kiện đầu vào có phải là một sự kiện phím không và gán nó vào biến key_event.
            if let Event::Key(key_event) = event::read()? {
                // kiểm tra mã phím và xử lý các sự kiện cụ thể. 
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            // audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        // Draw & render
        player.draw(&mut curr_frame);
        // Gửi khung hình curr_frame cho quá trình vẽ và hiển thị (rendering)
        let _ = render_tx.send(curr_frame);
        // Tạm ngừng chương trình trong 1 mili giây để kiểm soát tốc độ lặp (thường được sử dụng để kiểm soát tốc độ khung hình).
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    // audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
