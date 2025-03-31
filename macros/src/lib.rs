// macros/src/lib.rs

extern crate proc_macro;

mod call;
mod runtime;

use proc_macro::TokenStream;

/// Macro de atributo para gerar código de chamada de pallets
#[proc_macro_attribute]
pub fn call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    call::expand::expand_call(item)
}

/// Macro de atributo para gerar o runtime
#[proc_macro_attribute]
pub fn runtime(_attr: TokenStream, item: TokenStream) -> TokenStream {
    runtime::expand::expand_runtime(item)
}
