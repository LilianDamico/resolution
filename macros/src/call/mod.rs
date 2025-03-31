mod parse;
mod expand;

use proc_macro2::TokenStream;

/// Esta função é chamada diretamente pela macro #[macros::call]
pub fn call_macro(item: TokenStream) -> TokenStream {
    expand::expand_call(item)
}
