use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Meta, Path, Type, parse_macro_input};

#[proc_macro_derive(ControlledAccess, attributes(access))]
pub fn derive_controlled_access(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("ControlledAccess only supports structs with named fields"),
        },
        _ => panic!("ControlledAccess only supports structs"),
    };

    let field_permissions: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let (role_paths, field_type) = extract_role_paths_and_type_from_field(field);
            (field_name.clone(), role_paths, field_type.clone())
        })
        .collect();

    let struct_name = &input.ident;
    let accessor_combinations = get_accessor_combinations(struct_name, field_permissions);

    // Collect all accessor struct definitions
    let accessor_structs_and_impls =
        accessor_combinations
            .iter()
            .map(|(accessor_name, accessor_fields)| {
                let field_definitions = accessor_fields.iter().map(|(field_name, field_type)| {
                    quote! {
                        #field_name: #field_type,
                    }
                });

                let field_access_definitions =
                    accessor_fields.iter().map(|(field_name, field_type)| {
                        quote! {
                            pub fn #field_name(&self) -> &#field_type {
                                &self.#field_name
                            }
                        }
                    });

                let new_method_definition_mapping =
                    accessor_fields.iter().map(|(field_name, _)| {
                        quote! {
                            #field_name: orig.#field_name
                        }
                    });

                let accessor_name_ident =
                    proc_macro2::Ident::new(&accessor_name, proc_macro2::Span::call_site());

                quote! {
                    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
                    pub struct #accessor_name_ident {
                        #(#field_definitions)*
                    }

                    impl #accessor_name_ident {

                        pub fn new(orig: #struct_name) -> Self {
                            Self {
                                #(#new_method_definition_mapping,)*
                            }
                        }

                        #(#field_access_definitions)*
                    }
                }
            });

    // Combine everything into the final output
    let expanded = quote! {
        #(#accessor_structs_and_impls)*
    };

    TokenStream::from(expanded)
}

fn get_accessor_combinations(
    struct_ident: &proc_macro2::Ident,
    field_permissions: Vec<(syn::Ident, Vec<Path>, Type)>,
) -> HashMap<String, Vec<(proc_macro2::Ident, Type)>> {
    let mut accessor_combinations: HashMap<String, Vec<(proc_macro2::Ident, Type)>> =
        HashMap::new();
    // figure out all combinations of accessors needed
    for (field_name, roles, field_type) in field_permissions {
        for role in roles {
            // build a name
            let accessor_name = format!(
                "{}{}Accessor",
                struct_ident.to_string(),
                role.segments.last().unwrap().ident.to_string()
            );
            // add the field
            match accessor_combinations.get_mut(&accessor_name) {
                Some(field_list) => {
                    field_list.push((field_name.clone(), field_type.clone()));
                }
                None => {
                    accessor_combinations.insert(
                        accessor_name,
                        vec![(field_name.clone(), field_type.clone())],
                    );
                }
            }
        }
    }
    accessor_combinations
}

fn extract_role_paths_and_type_from_field(field: &Field) -> (Vec<Path>, &Type) {
    let mut role_paths = Vec::new();

    // Extract the field type
    let field_type = &field.ty;

    for attr in &field.attrs {
        if attr.path().is_ident("access") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    let tokens = &meta_list.tokens;
                    struct PathList(syn::punctuated::Punctuated<Path, syn::Token![,]>);
                    impl syn::parse::Parse for PathList {
                        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                            Ok(PathList(syn::punctuated::Punctuated::parse_terminated(
                                input,
                            )?))
                        }
                    }
                    if let Ok(path_list) = syn::parse2::<PathList>(tokens.clone()) {
                        for path in path_list.0 {
                            role_paths.push(path);
                        }
                    }
                }
                _ => panic!("role attribute must be a list"),
            }
        }
    }

    (role_paths, field_type)
}
