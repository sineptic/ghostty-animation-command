use std::{
    io::{stdout, Read, Write},
    process::exit,
    time::SystemTime,
};

use animation::FRAMES;

mod animation;

fn main() {
    let _wrapper = alternate_screen_wrapper::AlternateScreen::enter().unwrap();

    std::thread::spawn(move || {
        let mut stdin = std::io::stdin();
        let mut buf = [0; 1024];
        loop {
            let count = stdin.read(&mut buf).unwrap();
            if buf[..count].contains(&b'q') {
                break;
            }
        }
        let result = alternate_screen_wrapper::restore_terminal();
        if let Err(err) = result {
            eprintln!("Error: {err}");
            exit(1);
        }
        exit(0);
    });

    let start = SystemTime::now();
    const MICROS_PER_FRAME: u64 = 30_000;

    let mut stdout = stdout();
    crossterm::execute!(
        stdout,
        crossterm::cursor::Hide,
        crossterm::terminal::DisableLineWrap
    )
    .unwrap();
    loop {
        // Automatically skip frames when render is slow
        let frame_number =
            (start.elapsed().unwrap().as_micros() / MICROS_PER_FRAME as u128) as usize;
        let frame = &FRAMES[frame_number % FRAMES.len()];
        let (width, height) = crossterm::terminal::size().unwrap();
        let width_gap = width.saturating_sub(animation::IMAGE_WIDTH) / 2;
        let height_gap = height.saturating_sub(animation::IMAGE_HEIGHT) / 2;

        crossterm::execute!(
            stdout,
            crossterm::terminal::BeginSynchronizedUpdate,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )
        .unwrap();
        for (i, line) in frame
            .iter()
            .enumerate()
            .take_while(|(i, _line)| *i as u16 <= height)
        {
            crossterm::execute!(
                stdout,
                crossterm::cursor::MoveTo(width_gap, i as u16 + height_gap)
            )
            .unwrap();
            stdout.write_all(line.to_string().as_bytes()).unwrap();
        }
        crossterm::execute!(stdout, crossterm::terminal::EndSynchronizedUpdate).unwrap();

        std::thread::sleep(std::time::Duration::from_micros(MICROS_PER_FRAME));
    }
}
