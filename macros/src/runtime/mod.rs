// macros/src/runtime/mod.rs

pub mod expand;
pub mod parse;

use proc_macro::TokenStream;

/// Chamado pela macro #[macros::runtime]
pub fn runtime_macro(item: TokenStream) -> TokenStream {
    expand::expand_runtime(item)
}

