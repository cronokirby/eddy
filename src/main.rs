extern crate termion;

use std::io;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

/// Holds the state for interacting with a terminal.
///
/// This encapsulates the creation of a raw terminal we can write to, and use
/// primitive operations on. The terminal acts as a kind of "screen", that we
/// can manipulate and blit text onto
struct Terminal {
    /// This holds the writable interface we use.
    ///
    /// We manipulate our terminal by writing special text to it.
    stdout: RawTerminal<Stdout>,
}

impl Terminal {
    /// Create a new raw terminal tied to stdout.
    fn stdout() -> io::Result<Terminal> {
        Ok(Terminal {
            stdout: stdout().into_raw_mode()?,
        })
    }

    /// Clear the entire terminal screen.
    fn clear(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}", termion::clear::All)
    }

    /// Move the cursor to a specific location.
    fn goto(&mut self, line: u16, col: u16) -> io::Result<()> {
        write!(self.stdout, "{}", termion::cursor::Goto(line, col))
    }
}

fn main() -> io::Result<()> {
    let stdin = stdin();
    let mut term = Terminal::stdout()?;
    for c in stdin.keys() {
        term.clear()?;
        term.goto(1, 1)?;
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            _ => {}
        }
    }
    Ok(())
}
