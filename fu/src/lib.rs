#![feature(generic_associated_types)]
pub mod m;
pub type B<T> = Option<T>;

#[test]
fn test_bit() {
  let _0:B<u8> = Some(255);
  assert!(_0.is_some());
}
