use std::{fs, io};

#[cfg(target_os = "linux")]
#[test]
fn on_linux() {
  let output = std::process::Command::new("uname")
    .output()
    .expect("should be linux!");
  assert_eq!(b"Linux\n", output.stdout.as_slice());
}

#[cfg(target_os = "linux")]
#[test]
fn io_uring_result() -> io::Result<()> {
  use std::os::unix::io::AsRawFd;

  use io_uring::{opcode, types, IoUring};

  let mut ring = IoUring::new(8)?;

  let fd = fs::File::open("/dev/null")?;
  let mut buf = vec![0; 1024];

  let read_e = opcode::Read::new(types::Fd(fd.as_raw_fd()), buf.as_mut_ptr(), buf.len() as _)
    .build()
    .user_data(0x42);

  // Note that the developer needs to ensure
  // that the entry pushed into submission queue is valid (e.g. fd, buffer).
  unsafe {
    ring
      .submission()
      .push(&read_e)
      .expect("submission queue is full");
  }

  ring.submit_and_wait(1)?;

  let cqe = ring.completion().next().expect("completion queue is empty");

  assert_eq!(cqe.user_data(), 0x42);
  assert!(cqe.result() >= 0, "read error: {}", cqe.result());

  Ok(())
}
