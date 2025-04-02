use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl, ImplItem, ImplItemFn};

pub fn parse_impl(input: TokenStream) -> TokenStream {
    let impl_block: ItemImpl = parse_macro_input!(input as ItemImpl);

    for item in &impl_block.items {
        if let ImplItem::Fn(ImplItemFn { sig, .. }) = item {
            println!("Função encontrada: {}", sig.ident);
        }
    }

    quote::quote! {
        #impl_block
    }
    .into()
}
