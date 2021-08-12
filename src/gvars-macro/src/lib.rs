use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemConst, AttributeArgs};
use darling::FromMeta;
use quote::quote;

#[proc_macro_attribute]
pub fn gvar(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as ItemConst);

    #[derive(Debug, FromMeta)]
    struct Args {
        #[darling(multiple)]
        alias: Vec<String>,
    }
    let args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let visibility = input.vis;
    let name = input.ident;
    let ty = input.ty;
    let init = input.expr;
    let alias = args.alias;

    TokenStream::from(quote! {
        gvars::make!(#visibility, #name, #ty, #init, #(#alias),*);
    })
}
