use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::{ImplItem, ImplItemFn, ItemImpl};

pub fn parse_runtime_def(ast: &ItemImpl) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut enum_variants = vec![];
    let mut match_arms = vec![];

    for item in &ast.items {
        if let ImplItem::Fn(ImplItemFn { sig, .. }) = item {
            let method_name: &Ident = &sig.ident;

            let variant = quote! {
                #method_name
            };

            let match_arm = quote! {
                RuntimeCall::#method_name => {
                    self.#method_name(caller)?;
                }
            };

            enum_variants.push(variant);
            match_arms.push(match_arm);
        }
    }

    (enum_variants, match_arms)
}
