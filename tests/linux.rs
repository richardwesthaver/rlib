#[cfg(target_os = "linux")]
#[test]
fn on_linux() {
  let output = std::process::Command::new("uname")
    .output()
    .expect("should be linux!");
  assert_eq!(b"Linux\n", output.stdout.as_slice());
}
