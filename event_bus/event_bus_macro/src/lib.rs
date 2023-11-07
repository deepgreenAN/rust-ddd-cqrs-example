use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Event, attributes(event_bus))]
pub fn derive_event(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    derive_event_inner(input_ast)
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

fn derive_event_inner(input_ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input_ast.ident;

    // ライフタイム・型パラメーターを持つ場合はエラーを返す
    if !input_ast.generics.params.is_empty() {
        return Err(syn::Error::new(
            input_ast.generics.span(),
            "Event does not allow generic or lifetime params.".to_string(),
        ));
    }

    Ok(quote! {
        impl ::event_bus::Event for #type_name {}
    })
}
