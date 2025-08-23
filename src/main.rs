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

    // hide cursor
    stdout.write_all(b"\x1B[?25l").unwrap();
    // disable line wrap
    stdout.write_all(b"\x1B[?7l").unwrap();
    loop {
        // Automatically skip frames when render is slow
        let frame_number =
            (start.elapsed().unwrap().as_micros() / MICROS_PER_FRAME as u128) as usize;
        let frame = &FRAMES[frame_number % FRAMES.len()];
        // TODO: use syscall to get size of /dev/tty
        let (width, height) = crossterm::terminal::size().unwrap();
        let width_gap = width.saturating_sub(animation::IMAGE_WIDTH) / 2;
        let height_gap = height.saturating_sub(animation::IMAGE_HEIGHT) / 2;

        // clear screen
        stdout.write_all(b"\x1B[2J").unwrap();

        for (i, line) in frame
            .iter()
            .enumerate()
            .take_while(|(i, _line)| *i as u16 <= height)
        {
            // move cursor
            stdout
                .write_fmt(format_args!(
                    "\x1B[{};{}H",
                    i as u16 + height_gap + 1, // height
                    width_gap + 1,             // width
                ))
                .unwrap();
            stdout.write_all(line.to_string().as_bytes()).unwrap();
        }
        stdout.flush().unwrap();

        std::thread::sleep(std::time::Duration::from_micros(MICROS_PER_FRAME));
    }
}
