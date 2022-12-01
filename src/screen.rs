use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    layout::Rect,
    style::{Color, Style},
    Terminal,
};

const WIDTH: usize = 32;
const HEIGHT: usize = 64;

pub struct Screen {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    scale: u32,
    width: usize,
    height: usize,
    pub screen: [[u8; HEIGHT]; WIDTH],
}

impl Screen {
    pub fn build() -> Result<Self, std::io::Error> {
        let terminal = Screen::setup_terminal()?;

        Ok(Self {
            terminal,
            scale: 4,
            width: WIDTH,
            height: HEIGHT,
            screen: [[0; HEIGHT]; WIDTH],
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

    pub fn restore_terminal(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

            for x in 0..WIDTH {
                for y in 0..HEIGHT {
                    if self.screen[x][y] != 1 { continue; }
                    let size = Rect::new(x as u16, y as u16, self.scale as u16, self.scale as u16);
                    let block = Block::default().style(Style::default().bg(Color::White));
                    f.render_widget(block, size);
                }
            }
        }).unwrap();
    }

    pub fn draw_pixel(&mut self, x: u16,  y: u16) -> bool {
        let x = (x % WIDTH as u16) as usize;
        let y = (y % HEIGHT as u16) as usize;

        self.screen[y][x] ^= 1;

        self.screen[y][x] != 1
    }

    pub fn clear(&mut self) {
        self.screen = [[0; HEIGHT]; WIDTH];
    }
}
