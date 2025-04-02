// macros/src/call/mod.rs

pub mod expand;
pub mod parse;

use proc_macro::TokenStream;

/// Chamado pela macro #[macros::call]
pub fn call_macro(item: TokenStream) -> TokenStream {
    expand::expand_call(item)
}
