//! eve - event engines
use std::{os::unix::io::RawFd, ptr};

pub use io_uring;
use io_uring::{opcode, squeue, types, SubmissionQueue};

#[derive(Clone, Debug)]
pub enum Token {
  Accept,
  Poll {
    fd: RawFd,
  },
  Read {
    fd: RawFd,
    buf_index: usize,
  },
  Write {
    fd: RawFd,
    buf_index: usize,
    offset: usize,
    len: usize,
  },
}

pub struct AcceptCount {
  entry: squeue::Entry,
  count: usize,
}

impl AcceptCount {
  pub fn new(fd: RawFd, token: usize, count: usize) -> AcceptCount {
    AcceptCount {
      entry: opcode::Accept::new(types::Fd(fd), ptr::null_mut(), ptr::null_mut())
        .build()
        .user_data(token as _),
      count,
    }
  }

  pub fn push_to(&mut self, sq: &mut SubmissionQueue<'_>) {
    while self.count > 0 {
      unsafe {
        match sq.push(&self.entry) {
          Ok(_) => self.count -= 1,
          Err(_) => break,
        }
      }
    }

    sq.sync();
  }
}

#[cfg(test)]
#[test]
fn bench() {}
