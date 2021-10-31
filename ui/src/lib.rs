//! ui library
use crossterm::{cursor, execute, queue, style, terminal, Result};
use std::{
  io::{stdout, BufWriter, Stdout, Write},
  time::{Duration, Instant},
};

pub struct Ui {
  pub canvas: Vec<Vec<char>>,
  pub locations: Vec<usize>,
  pub length: Vec<usize>,
  pub time: Vec<(Instant, Duration)>,
  pub queue: Vec<usize>,
}

pub fn clear(w: &mut BufWriter<Stdout>) -> Result<()> {
  queue!(w, terminal::Clear(terminal::ClearType::All))?;
  Ok(())
}

pub fn run() -> Result<()> {
  let mut stdout = BufWriter::with_capacity(8_192, stdout()); //?
  let (width, height) = terminal::size()?;
  let h = height as usize;

  let mut is_running = true;

  terminal::enable_raw_mode()?;
  execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
  while is_running {
    stdout.flush()?;
  }

  execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
