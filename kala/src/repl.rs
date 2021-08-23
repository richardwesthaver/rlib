use crate::Result;
use rustyline::Editor;

/// test REPL
pub fn echo() -> Result<()> {
  let mut rl = Editor::<()>::new();
  let line = rl.readline("> ")?; // read
  println!("Line: {}", line); // eval / print
  Ok(())
}
