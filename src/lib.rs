use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input,
    DataEnum,
    Data,
    DeriveInput,
    Fields
};

#[proc_macro_derive(QuickFrom, attributes(quick_from))]
pub fn quick_from(input: TokenStream) -> TokenStream {
    let DeriveInput{ident, data, generics, ..} = parse_macro_input!(input);
    let generics = generics.to_token_stream();

    let variants = if let Data::Enum(DataEnum{variants, ..}) = data {
        variants
    } else {
        return quote!{ compile_error!("QuickFrom only accepts enums") }.into()
    };

    let mut out = TokenStream::new();

    for variant in variants {
        let has_attr = variant.attrs.iter().find(|attr| {
            attr.path.get_ident().map_or(false, |id| {
                id == "quick_from"
            })
        }).is_some();

        if !has_attr {
            continue
        }

        let enum_type = &ident;
        let var_name = variant.ident;

        let var_type = if let Fields::Unnamed(fields) = variant.fields {
            if fields.unnamed.len() != 1 {
                return quote!{
                    compile_error!("QuickFrom #[quick_from] variant must have \
                        exactly one unnamed field")
                }.into()
            }

            fields.unnamed.first().unwrap().ty.clone()
        } else {
            return quote!{
                compile_error!("QuickFrom #[quick_from] variant must have \
                    exactly one unnamed field")
            }.into()
        };

        let x : TokenStream = quote!{
            impl#generics From<#var_type> for #enum_type#generics {
                fn from(x : #var_type) -> Self {
                    Self::#var_name(x)
                }
            }
        }.into();

        out.extend(x);
    }

    out.into()
}
