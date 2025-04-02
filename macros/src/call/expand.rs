use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

pub fn expand_call(item: TokenStream) -> TokenStream {
    let item_proc_macro: TokenStream2 = item.into();
    let ast: ItemImpl = parse_macro_input!(item_proc_macro as ItemImpl);

    // Aqui você insere a lógica de expansão real
    let expanded = quote! {
        #ast
    };

    TokenStream::from(expanded)
}
