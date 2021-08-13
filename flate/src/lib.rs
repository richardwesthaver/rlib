use std::{
  fs, io,
  path::{Path, PathBuf},
};

pub fn pack(src: &Path, dst: &Path) {
  let mut tar = tar::Builder::new(Vec::new());
  tar.append_dir_all(PathBuf::from(src), src).unwrap();
  let tar = tar.into_inner().unwrap();
  let file = fs::File::create(dst.join(src.with_extension("tar.zst"))).unwrap();
  zstd::stream::copy_encode(&tar[..], file, 22).unwrap(); // ultramaximum
}

/// unpack a file
pub fn unpack(src: &Path, dst: &Path) {
  let input = fs::File::open(src).unwrap();
  let mut buff = Vec::new();
  zstd::stream::copy_decode(input, &mut buff).unwrap();
  let mut ar = tar::Archive::new(&buff[..]);
  ar.unpack(dst).unwrap();
}

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
  pack(Path::new("src"), Path::new("."));
  unpack(Path::new("src.tar.zst"), Path::new("."));
  unpack_replace(Path::new("src.tar.zst"), Path::new("."));
}
