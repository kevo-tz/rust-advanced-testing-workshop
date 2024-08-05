use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Attribute, ItemFn};

#[proc_macro_attribute]
pub fn vanilla_test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut test_fn: ItemFn = syn::parse_macro_input!(input as ItemFn);
    if test_fn
        .attrs
        .iter()
        .find(|a| is_test_attribute(a))
        .is_some()
    {
        test_fn.to_token_stream()
    } else {
        quote::quote! {
            #[test]
            #test_fn
        }
    }
    .into()
}

fn is_test_attribute(attr: &Attribute) -> bool {
    let Some(seg) = attr.path().segments.last() else {
        return false;
    };
    seg.ident == "test"
}
