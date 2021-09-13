//! dmc
#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod tests;

lalrpop_mod!(pub calc); // synthesized by LALRPOP

#[test]
fn test_calc() {
    assert!(calc::TermParser::new().parse("22").is_ok());
    assert!(calc::TermParser::new().parse("(22)").is_ok());
    assert!(calc::TermParser::new().parse("((((22))))").is_ok());
    assert!(calc::TermParser::new().parse("((22)").is_err());
}
