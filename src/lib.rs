use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, ItemStruct, Path, parse_macro_input};

mod data;

#[proc_macro_derive(ControlledAccess, attributes(access))]
pub fn derive_controlled_access(input: TokenStream) -> TokenStream {
    data::derive_controlled_access_internal(input)
}

#[proc_macro_derive(AccessRoles)]
pub fn derive_access_roles(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Extract enum variants
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("AccessRoles can only be used on enums"),
    };

    // Generate marker structs for each variant
    let marker_structs = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let marker_name = Ident::new(
            &format!("{}Marker", variant_name),
            proc_macro2::Span::call_site(),
        );

        quote! {
            pub struct #marker_name;
        }
    });

    let expanded = quote! {
        #(#marker_structs)*
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn access_guard(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let role_path = parse_macro_input!(attrs as Path);
    let mut input_struct = parse_macro_input!(input as ItemStruct);

    let marker_type = Ident::new(
        &format!("{}Marker", role_path.segments.last().unwrap().ident),
        proc_macro2::Span::call_site(),
    );

    match &mut input_struct.fields {
        Fields::Named(fields) => {
            let new_field: syn::Field = syn::parse_quote! {
                #[doc(hidden)]
                pub _role_marker: std::marker::PhantomData<#marker_type>
            };
            fields.named.push(new_field);
        }
        Fields::Unit => {
            // Convert unit struct to named fields
            let new_field: syn::Field = syn::parse_quote! {
                #[doc(hidden)]
                pub _role_marker: std::marker::PhantomData<#marker_type>
            };
            let mut named_fields = syn::punctuated::Punctuated::new();
            named_fields.push(new_field);

            input_struct.fields = Fields::Named(syn::FieldsNamed {
                brace_token: syn::token::Brace::default(),
                named: named_fields,
            });
        }
        _ => panic!("Unsupported field type"),
    }
    TokenStream::from(quote! { #input_struct })
}
