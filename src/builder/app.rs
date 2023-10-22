use std::io::{self, stderr};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Alignment, CrosstermBackend},
    widgets::{Paragraph, Wrap},
    Terminal,
};

use crate::vfs_parser::VfsParser;

#[allow(dead_code)]
const INSTRUCTIONS: &'static str = concat!(
    "virtree: Builder mode.\n\n",
    "Input paths to grow the virtual directory tree, ",
    "or press the Esc key to quit. Type \":help\" ",
    "for a list of all commands.",
);

// Define Frame type for convenience (using stderr)
pub(self) type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

#[allow(dead_code)]
pub(crate) struct App<'a> {
    parser: &'a VfsParser,
    should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(parser: &'a VfsParser) -> Self {
        Self { 
            parser, 
            should_quit: false,
        }
    }

    /// Prepare the terminal for control.
    fn startup(&mut self) -> io::Result<()> {
        stderr().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(())
    }

    /// Return terminal to normal state.
    pub fn shutdown(&mut self) -> io::Result<()> {
        stderr().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn render(&mut self, f: &mut Frame<'_>) {
        let area = f.size();

        f.render_widget(
            Paragraph::new(INSTRUCTIONS)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    fn update(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                    self.should_quit = true;
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.startup()?;

        let mut term = Terminal::new(CrosstermBackend::new(stderr()))?;
        term.clear()?; // Clear the entire screen.

        loop {
            term.draw(|f| {
                self.render(f);
            })?;

            self.update()?;

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }
}
