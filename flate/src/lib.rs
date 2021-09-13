//! flate compression/archival modules
//!
//! This library provides wrappers for compression and archive
//! libraries. Currently only zstd and tar are supported, but will
//! soon support a wider variety of backends.
//!
//! Backends will be conditionally-compiled based on feature flags,
//! and will rely on C bindings as little as possible, allowing for
//! more flexibility in platform support.
use std::{
  fs, io,
  path::{Path, PathBuf},
};

/// Level of compression data should be compressed with.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Level {
    /// Fastest quality of compression, usually produces bigger size.
    Fastest,
    /// Best quality of compression, usually produces the smallest size.
    Best,
    /// Default quality of compression defined by the selected compression algorithm.
    Default,
    /// Precise quality based on the underlying compression algorithms'
    /// qualities. The interpretation of this depends on the algorithm chosen
    /// and the specific implementation backing it.
    /// Qualities are implicitly clamped to the algorithm's maximum.
    Precise(u32),
}

impl Level {
  fn into_zstd(self) -> i32 {
    match self {
      Self::Fastest => 1,
      Self::Best => 21,
      Self::Precise(quality) => quality.min(21) as i32,
      Self::Default => 0,
    }
  }
}
/// Pack a SRC directory, and return a SRC.tar.zst compressed archive at DST.
pub fn pack(src: &Path, dst: &Path, level: Option<Level>) {
  let mut tar = tar::Builder::new(Vec::new());
  tar.append_dir_all(PathBuf::from(src), src).unwrap();
  let tar = tar.into_inner().unwrap();
  let file = fs::File::create(dst.join(src.with_extension("tar.zst"))).unwrap();
  zstd::stream::copy_encode(&tar[..], file, level.unwrap_or(Level::Best).into_zstd()).unwrap(); // ultramaximum
}

/// unpack a tar.zst compressed archive
pub fn unpack(src: &Path, dst: &Path) {
  let input = fs::File::open(src).unwrap();
  let mut buff = Vec::new();
  zstd::stream::copy_decode(input, &mut buff).unwrap();
  let mut ar = tar::Archive::new(&buff[..]);
  ar.unpack(dst).unwrap();
}

/// unpack a tar.zst compressed archive, removing the source file before
/// returning
pub fn unpack_replace(src: &Path, dst: &Path) {
  unpack(src, dst);
  fs::remove_file(src).expect("could not remove source package");
}
/// compress a file with zstd
pub fn compress(source: &Path) -> io::Result<()> {
  let mut file = fs::File::open(source)?;
  let mut encoder = {
    let target = fs::File::create(source.with_extension("zst"))?;
    zstd::Encoder::new(target, 22)?
  };
  io::copy(&mut file, &mut encoder)?;
  encoder.finish()?;
  Ok(())
}

/// decompress a zst file
pub fn decompress(source: &Path) -> io::Result<()> {
  let mut decoder = {
    let file = fs::File::open(source)?;
    zstd::Decoder::new(file)?
  };
  let mut target = fs::File::create(source.to_str().unwrap().trim_end_matches(".zst"))?;
  io::copy(&mut decoder, &mut target)?;
  decoder.finish();
  Ok(())
}

#[test]
fn pack_test() {
  // this is extremely dangerous, did it for fun but c'mon now 8)
  pack(Path::new("src"), Path::new("."), None);
  unpack(Path::new("src.tar.zst"), Path::new("."));
  unpack_replace(Path::new("src.tar.zst"), Path::new("."));
}
