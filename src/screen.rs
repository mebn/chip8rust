use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use tui::{
    backend::CrosstermBackend,
    widgets::Block,
    layout::Rect,
    style::{Color, Style},
    Terminal,
};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Screen {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    scale: u32,
    pub screen: [[u8; WIDTH]; HEIGHT],
}

impl Screen {
    pub fn build() -> Result<Self, std::io::Error> {
        let terminal = Screen::setup_terminal()?;

        Ok(Self {
            terminal,
            scale: 1,
            screen: [[0; WIDTH]; HEIGHT],
        })
    }

    fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>, std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    fn restore_terminal(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    pub fn draw(&mut self) {
        self.terminal.draw(|f| {
            f.render_widget(Block::default().style(Style::default().bg(Color::Black)), f.size());

            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    if self.screen[y][x] != 1 { continue; }

                    let block = Block::default().style(Style::default().bg(Color::White));
                    let size = Rect::new(
                        x as u16 * self.scale as u16 * 2,
                        y as u16 * self.scale as u16,
                        self.scale as u16 * 2,
                        self.scale as u16
                    );

                    f.render_widget(block, size);
                }
            }
        }).unwrap();
    }

    pub fn set_pixel(&mut self, x: u16,  y: u16) -> bool {
        let x = x as usize % WIDTH as usize;
        let y = y as usize % HEIGHT as usize;

        self.screen[y][x] ^= 1;

        self.screen[y][x] != 1
    }

    pub fn clear(&mut self) {
        self.screen = [[0; WIDTH]; HEIGHT];
    }
}

impl std::ops::Drop for Screen {
    fn drop(&mut self) {
        self.restore_terminal().unwrap();
    }
}
