use rustyline::Editor;

use crate::Result;

/// Minimal REPL
pub fn repl() -> Result<()> {
  let mut rl = Editor::<()>::new();
  loop {
    let line = rl.readline("> ")?; // read
    println!("Line: {}", line); // eval / print
  } // loop
}
