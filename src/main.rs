mod screen;
mod controls;
mod chip8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screen = screen::Screen::build()?;
    let controls = controls::Controls::new();
    let mut chip8 = chip8::Chip8::new(screen, controls);

    let rom = std::fs::read("./roms/BLITZ").unwrap();
    chip8.load_rom(rom);

    let tick_rate = std::time::Duration::from_millis(1);
    let mut last_tick = std::time::Instant::now();

    // event loop
    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if let crossterm::event::KeyCode::Char('p') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if chip8.is_paused {
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            chip8.cycle();

            last_tick = std::time::Instant::now();
        }
    }
}
