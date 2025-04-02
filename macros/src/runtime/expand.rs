use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

use crate::runtime::parse::parse_runtime_def;

pub fn expand_runtime(item: TokenStream) -> TokenStream {
    let item_proc_macro: TokenStream2 = item.into();
    let ast: ItemImpl = parse_macro_input!(item_proc_macro as ItemImpl);

    let (enum_variants, match_arms) = parse_runtime_def(&ast);

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        pub enum RuntimeCall {
            #(#enum_variants),*
        }

        impl Runtime {
            pub fn dispatch(&mut self, caller: u64, call: RuntimeCall) -> support::DispatchResult {
                match call {
                    #(#match_arms)*
                }
                Ok(())
            }
        }

        #ast
    };

    TokenStream::from(expanded)
}
