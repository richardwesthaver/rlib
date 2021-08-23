use crate::Result;
use midir::{Ignore, MidiInput, MidiOutput};

/// detect available midi devices and print a summary
pub fn list_midi_ports() -> Result<()> {
  let mut midi_in = MidiInput::new("midir test input")?;
  midi_in.ignore(Ignore::None);
  let midi_out = MidiOutput::new("midir test output")?;

  println!("Available input ports:");
  for (i, p) in midi_in.ports().iter().enumerate() {
    println!("{}: {}", i, midi_in.port_name(p)?);
  }

  println!("\nAvailable output ports:");
  for (i, p) in midi_out.ports().iter().enumerate() {
    println!("{}: {}", i, midi_out.port_name(p)?);
  }

  Ok(())
}
