use quote::quote;
use proc_macro2::TokenStream;
use crate::call::parse::CallArgs;

/// Função responsável por gerar código a partir dos dados analisados pela macro
pub fn expand_call(item: TokenStream) -> TokenStream {
    // Usa a função parse de CallArgs para extrair dados da função original
    let args: CallArgs = syn::parse2(item.clone()).unwrap();
    let name = args.name;

    // Gera um nome de enum usando o nome da função
    let enum_name = quote::format_ident!("{}", name.to_string().to_uppercase());

    // Gera um novo TokenStream com a definição do enum + função original
    quote! {
        // Define uma enumeração com a função como uma variante
        pub enum Call {
            #enum_name,
        }

        // Reinsere a função original no código compilado
        #item
    }
}
