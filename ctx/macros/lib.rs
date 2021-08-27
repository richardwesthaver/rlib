//! tokio `main` and `test` macros
#![allow(elided_lifetimes_in_paths)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

use self::expand::{expand, Mode};

// Expand from:
//
//     #[ctx::main]
//     fn main(fb: CtxInit) {
//         ...
//     }
//
// to:
//
//     fn main() {
//         let : CtxInit = ctxinit::r#impl::perform_init();
//         ...
//     }
//
// If async, also add a #[tokio::main] attribute.
//
// Accepts optional attribute argument disable_fatal_signals to disable adding
// handler to fatal signals in perform_init().
// Argument must be one of `default`, `none`, `all`, `sigterm_only`
// that represents the signal bit mask. For  example, the following disables
// SIGTERM:
//
//      #[ctx::main(disable_fatal_signals = sigterm_only)
//
// - `default`: disables SIGTERM and SIGINT, and is also the default if
//   `disable_fatal_signals`
//  is not specified
// - `none`: disabled no signals, overrides the default
// - `all`: disables ALL signals
// - `sigterm_only`: disabled SIGTERM
#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
  expand(
    Mode::Main,
    parse_macro_input!(args with Punctuated::parse_terminated),
    parse_macro_input!(input),
  )
  .unwrap_or_else(|err| err.to_compile_error())
  .into()
}

// Same thing, expand:
//
//     #[ctx::test]
//     fn name_of_test(ctx: CtxInit) {
//         ...
//     }
//
// to:
//
//     #[test]
//     fn name_of_test() {
//         let ctx: CtxInit = ctxinit::r#impl::perform_init();
//         ...
//     }
//
// with either #[test] or #[tokio::test] attribute.
//
// Accepts optional attribute argument disable_fatal_signals to disable adding
// handler to fatal signals in perform_init().
// Argument must be an int literal that represents the signal bit mask. For
// example, the following disables SIGTERM:
//
//      #[ctx::main(disable_fatal_signals = 0x8000)
#[proc_macro_attribute]
pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
  expand(
    Mode::Test,
    parse_macro_input!(args with Punctuated::parse_terminated),
    parse_macro_input!(input),
  )
  .unwrap_or_else(|err| err.to_compile_error())
  .into()
}
