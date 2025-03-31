use proc_macro::TokenStream;

/// Gera automaticamente o código de roteamento do runtime.
pub fn expand_runtime(_item: TokenStream) -> TokenStream {
    // Também retorna sem modificação.
    // Em tutoriais avançados, isso cria enums e matches automáticos.
    _item
}
