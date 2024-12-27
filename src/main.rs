mod animation;

use std::io::stdout;
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{event, ExecutableCommand};

fn main() {
    setup_terminal();
    let color_primary = "\x1b[0m";
    let color_second = "\x1b[94m";
    loop {
        for frame in animation::ANIMATION_DATA.iter() {
            render_frame(frame, color_primary, color_second);
            key_press(30);
        }
    }
}

fn setup_terminal() {
    stdout().execute(EnterAlternateScreen).unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().execute(Hide).unwrap();
    enable_raw_mode().unwrap();
}

fn render_frame(frame: &[&str], color_primary: &str, color_second: &str) {
    let mut y = 0;
    let (width, _) = size().unwrap();
    let x = if width > 100 { (width - 100) / 2 } else { 0 };

    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();

    for &line in frame.iter() {
        let colored_line = line
            .replace("<color>", color_second)
            .replace("</color>", color_primary);
        stdout.execute(MoveTo(x, y)).unwrap();
        stdout.execute(Print(colored_line)).unwrap();
        y += 1;
    }
}

fn key_press(poll: u64) {
    if event::poll(Duration::from_millis(poll)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if let KeyCode::Char('q') = key_event.code {
                cleanup_terminal();
                std::process::exit(0);
            }
        }
    }
}

fn cleanup_terminal() {
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
    stdout().execute(Show).unwrap();
}
