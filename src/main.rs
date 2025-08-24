use std::{
    fs::File,
    io::{stdout, Read, Write},
    process::exit,
    time::Instant,
};

use animation::FRAMES;

mod animation;

fn main() {
    let wrapper = alternate_screen_wrapper::unix::AlternateScreenOnStdout::enter()
        .unwrap()
        .unwrap();

    std::thread::spawn(handle_quit_event);

    let res = run_animation();
    drop(wrapper);
    if let Err(err) = res {
        eprintln!("Error: {err}");
    }
}

fn run_animation() -> Result<(), std::io::Error> {
    let start = Instant::now();
    const MICROS_PER_FRAME: u64 = 30_000;

    let mut stdout = stdout();

    // hide cursor
    stdout.write_all(b"\x1B[?25l")?;
    // disable line wrap
    stdout.write_all(b"\x1B[?7l")?;
    loop {
        // Automatically skip frames when render is slow
        let frame_number = (start.elapsed().as_micros() / MICROS_PER_FRAME as u128) as usize;
        let frame = &FRAMES[frame_number % FRAMES.len()];
        let rustix::termios::Winsize {
            ws_row: height,
            ws_col: width,
            ..
        } = rustix::termios::tcgetwinsize(File::open("/dev/tty")?)?;
        let width_gap = width.saturating_sub(animation::IMAGE_WIDTH) / 2;
        let height_gap = height.saturating_sub(animation::IMAGE_HEIGHT) / 2;

        // clear screen
        stdout.write_all(b"\x1B[2J")?;

        for (i, line) in frame
            .iter()
            .enumerate()
            .take_while(|(i, _line)| *i as u16 <= height)
        {
            // move cursor
            stdout.write_fmt(format_args!(
                "\x1B[{};{}H",
                i as u16 + height_gap + 1, // height
                width_gap + 1,             // width
            ))?;
            stdout.write_all(line.to_string().as_bytes())?;
        }
        stdout.flush()?;

        std::thread::sleep(std::time::Duration::from_micros(MICROS_PER_FRAME));
    }
}

fn handle_quit_event() {
    let mut stdin = std::io::stdin();
    let mut buf = [0; 1024];
    loop {
        let Ok(count) = stdin.read(&mut buf) else {
            break;
        };
        if buf[..count].contains(&b'q') {
            break;
        }
    }
    let result = alternate_screen_wrapper::unix::restore_terminal();
    if let Err(err) = result {
        eprintln!("Error: {err}");
        exit(1);
    }
    exit(0);
}
