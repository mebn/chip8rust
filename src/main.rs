mod screen;
mod controls;
mod chip8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut screen = screen::Screen::build()?;
    // let controls = controls::Controls::new();
    // let mut chip8 = chip8::Chip8::new(screen, controls);

    // let rom = std::fs::read("./roms/SPACE-INVADER").unwrap();
    // chip8.load_rom(rom);
    // chip8.system_loop(60);

    // test
    screen.screen[10][10] = 1;
    screen.screen[0][0] = 1;
    screen.draw();

    std::thread::sleep(std::time::Duration::from_secs(2));

    screen.restore_terminal()?;
    Ok(())
}
