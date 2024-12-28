mod animation;

use std::io::stdout;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
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

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    let key_press_tx = tx.clone();
    thread::spawn(move || {
        loop {
            key_press(30, &key_press_tx);
        }
    });

    thread::spawn(move || {
        loop {
            for frame in animation::ANIMATION_DATA.iter() {
                render_frame(&rx, frame, color_primary, color_second);
            }
        }
    }).join().unwrap();
}

fn setup_terminal() {
    stdout().execute(EnterAlternateScreen).unwrap();
    stdout().execute(Clear(ClearType::All)).unwrap();
    stdout().execute(Hide).unwrap();
    enable_raw_mode().unwrap();
}

fn render_frame(rx: &Receiver<()>, frame: &[&str], color_primary: &str, color_second: &str) {
    let mut y = 0;
    let (width, _) = size().unwrap();
    let x = if width > 100 { (width - 100) / 2 } else { 0 };

    let mut stdout = stdout();

    // for &line in frame.iter() {
    //     let colored_line = line
    //         .replace("<color>", color_second)
    //         .replace("</color>", color_primary);
    //     stdout.execute(MoveTo(x, y)).unwrap();
    //     stdout.execute(Print(colored_line)).unwrap();
    //     y += 1;
    // }

    let colored_frame = frame.iter().map(|&line|line
            .replace("<color>", color_second)
            .replace("</color>", color_primary)).collect::<Vec<String>>();

    rx.recv().unwrap();
    stdout.execute(Clear(ClearType::All)).unwrap();
    colored_frame.into_iter().for_each(|line|{
        stdout.execute(MoveTo(x, y)).unwrap();
        stdout.execute(Print(line)).unwrap();
        y += 1;
    });
    
}

fn key_press(poll: u64, tx: &Sender<()>) {
    if event::poll(Duration::from_millis(poll)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            if let KeyCode::Char('q') = key_event.code {
                cleanup_terminal();
                std::process::exit(0);
            }
        }
    }
    tx.send(()).unwrap();
}

fn cleanup_terminal() {
    disable_raw_mode().unwrap();
    stdout().execute(LeaveAlternateScreen).unwrap();
    stdout().execute(Show).unwrap();
}
