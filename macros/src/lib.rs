extern crate proc_macro;

mod call;
mod runtime;

use proc_macro::TokenStream;

/// Gera enum Call e lógica de dispatch para um pallet.
#[proc_macro_attribute]
pub fn call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    call::expand::expand_call(item)
}

/// Gera enum RuntimeCall e lógica de dispatch para o runtime.
#[proc_macro_attribute]
pub fn runtime(_attr: TokenStream, item: TokenStream) -> TokenStream {
    runtime::expand::expand_runtime(item)
}
