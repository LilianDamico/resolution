use syn::{ItemFn, Ident};
use syn::parse::{Parse, ParseStream};
use syn::Result;

/// Estrutura que armazenará as informações extraídas da função com a macro
pub struct CallArgs {
    pub name: Ident,
}

impl Parse for CallArgs {
    /// Esta função define como extrair os dados de entrada da macro
    fn parse(input: ParseStream) -> Result<Self> {
        // Analisa a função inteira (fn nome() { ... })
        let func: ItemFn = input.parse()?;

        // Pega o identificador (nome) da função
        let name = func.sig.ident.clone();

        // Retorna os dados que serão usados na geração
        Ok(CallArgs { name })
    }
}
