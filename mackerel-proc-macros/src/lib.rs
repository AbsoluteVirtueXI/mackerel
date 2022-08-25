/*
    Bellow just a P.O.C. and tests for code insertion in the AST
*/
use proc_macro::*;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn call_trace(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = syn::parse(input).unwrap();
    let fn_item = match &mut ast {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };
    fn_item.block.stmts.insert(
        0,
        syn::parse(quote!(println!("count me in");).into()).unwrap(),
    );

    use quote::ToTokens;
    ast.into_token_stream().into()
}
