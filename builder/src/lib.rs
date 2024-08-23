use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    let ident = parsed.ident;
    let builder_name = format!("{}Builder", ident);
    let builder_ident = syn::Ident::new(&builder_name, ident.span());

    let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = parsed.data
    else {
        panic!("Only struct can be applied");
    };
    
    let fields = named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote! {
            #ident: Option<#ty>
        }
    });
    
    let fields_init = named.iter().map(|f| {
        let ident = &f.ident;
        quote! {
            #ident: None
        }
    });

    let expanded = quote! {
        pub struct #builder_ident {
            #(#fields,)*
        }
        
        impl #ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#fields_init,)*
                }
            }
        }
    };

    expanded.into()
}
