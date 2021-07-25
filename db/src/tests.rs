fn get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_ [u8] {
  source.as_ref()
}
