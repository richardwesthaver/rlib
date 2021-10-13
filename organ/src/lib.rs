#[macro_use] extern crate lalrpop_util;

#[cfg(test)] mod tests;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[test]
fn test_calc() {
  use grammar as org;
  assert!(org::TermParser::new().parse("22").is_ok());
  assert!(org::TermParser::new().parse("(22)").is_ok());
  assert!(org::TermParser::new().parse("((((22))))").is_ok());
  assert!(org::TermParser::new().parse("((22)").is_err());
}
