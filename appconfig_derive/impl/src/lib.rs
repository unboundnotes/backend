use std::{collections::HashMap, fmt::Display};

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ExprAssign, Field, Lit};

const NAME: &str = "name";
const DEFAULT: &str = "default";
const DEFAULT_FN: &str = "default_fn";

enum MyLit {
    Lit(Lit),
}

impl Display for MyLit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit = match self {
            MyLit::Lit(lit) => lit,
        };
        match lit {
            Lit::Str(s) => write!(f, "{}", s.value()),
            Lit::Int(i) => write!(f, "{}", i.base10_digits()),
            Lit::Bool(b) => write!(f, "{}", b.value),
            Lit::ByteStr(s) => todo!("ByteStr"),
            Lit::Byte(b) => write!(f, "{}", b.value()),
            Lit::Char(c) => write!(f, "{}", c.value()),
            Lit::Float(fl) => write!(f, "{}", fl.base10_digits()),
            Lit::Verbatim(l) => write!(f, "{}", l.to_string()),
        }
    }
}

fn split_tts(tts: TokenStream2) -> Vec<TokenStream2> {
    let mut tts = tts.into_iter();
    let mut tts_vec = Vec::new();
    while let Some(tt) = tts.next() {
        if tt.to_string() == "," {
            continue;
        }
        let mut tts_ = Vec::new();
        tts_.push(tt);
        while let Some(tt) = tts.next() {
            if tt.to_string() == "," {
                break;
            }
            tts_.push(tt);
        }
        tts_vec.push(tts_.into_iter().collect());
    }
    tts_vec
}

fn parse_attrs<T>(fields: &Punctuated<Field, T>) -> HashMap<String, HashMap<String, String>> {
    let mut res = HashMap::new();
    for f in fields {
        let attr = match f.attrs.iter().find(|a| a.path.is_ident("appconfig")) {
            Some(a) => a,
            None => continue,
        };
        let mut field_attrs = HashMap::new();
        let g = match attr.tokens.clone().into_iter().next().unwrap() {
            proc_macro2::TokenTree::Group(g) => g,
            _ => panic!("expected group"),
        };
        let tts = split_tts(g.stream());
        for tt in tts {
            let res: Result<ExprAssign, syn::Error> = syn::parse2(tt);
            if res.is_ok() {
                let expr = res.unwrap();
                let key = match *expr.left {
                    syn::Expr::Path(p) => p.path.get_ident().unwrap().to_string(),
                    _ => panic!("expected path"),
                };
                let value = match *expr.right {
                    syn::Expr::Lit(l) => MyLit::Lit(l.lit).to_string(),
                    syn::Expr::Path(p) => p.path.get_ident().unwrap().to_string(),
                    _ => panic!("expected literal"),
                };
                field_attrs.insert(key, value);
            }
        }
        res.insert(f.ident.as_ref().unwrap().to_string(), field_attrs);
    }
    res
}

#[proc_macro_derive(AppConfig, attributes(appconfig))]
pub fn app_config(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let orig_name = ast.ident.clone();
    let name = syn::Ident::new(&format!("{}Builder", ast.ident), ast.ident.span());
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        _ => panic!("AppConfig can only be derived for structs"),
    };
    let fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        _ => panic!("AppConfig can only be derived for structs with named fields"),
    };

    // TODO: Implement the following attrs:
    // - config(sep = "...")
    // - config(prefix = "...")
    // - config(datasource = "...")
    let attrs = parse_attrs(&fields);

    // TODO: Implement special cases for the `std::collections` types, as well as `Option` and `AppConfig`

    // 1. Generate a `builder` struct, with all optional fields
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #name: std::option::Option<#ty>
        }
    });

    // 2. Assert that all the types implement either (FromStr and ToString) or AppConfig
    let assert_types = fields.iter().map(|f| {
        let ty = &f.ty;
        quote! {
            static_assertions::assert_impl_any!(#ty: std::str::FromStr, appconfig_derive::AppConfig);
            static_assertions::assert_impl_any!(#ty: std::string::ToString, appconfig_derive::AppConfig);
        }
    });

    // 3. Try to load the values from the data source
    // 3a. TODO: You should be able to specify a name for the key
    let read_from_data_src = fields.iter().map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let sname = name.to_string();
        let ty = &f.ty;
        let key = attrs.get(&sname).map(|m| m.get(NAME)).flatten().or(Some(&sname)).unwrap().to_uppercase();
        quote! {
            match data_src.get(#key) {
                Err(e) => return Err(appconfig_derive::AppConfigError::DatastoreError(e)),
                Ok(None) => {},
                Ok(Some(value)) => {
                    let value = value.parse::<#ty>().map_err(|e| appconfig_derive::AppConfigError::ParsingError(Box::new(e)))?;
                    builder.#name = Some(value);
                },
            }
        }
    });

    // 4. Try to load the values from the environment
    let read_from_env = fields.iter().map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let sname = name.to_string();
        let ty = &f.ty;
        let key = attrs.get(&sname).map(|m| m.get(NAME)).flatten().or(Some(&sname)).unwrap().to_uppercase();

        quote! {
            builder.#name = builder.#name.or(
                std::env::var(#key)
                    .ok()
                    .map(|value| value.parse::<#ty>().map_err(|e| appconfig_derive::AppConfigError::ParsingError(Box::new(e))))
                    .transpose()?
            );
        }
    });

    // 5. Try to load the values from the default option
    let read_from_default = fields.iter().filter_map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let sname = name.to_string();
        let ty = &f.ty;
        let key = attrs.get(&sname).map(|m| m.get(DEFAULT)).flatten()?;
        Some(quote! {
            builder.#name = builder.#name.or(
                Some(#key.parse::<#ty>().map_err(|e| appconfig_derive::AppConfigError::ParsingError(Box::new(e)))?)
            );
        })
    });

    // 5a. This should also allow default_fn to be specified
    let read_from_default_fn = fields.iter().filter_map(|f| {
        let name = &f.ident.as_ref().unwrap();
        let sname = name.to_string();
        let fn_name = attrs.get(&sname).map(|m| m.get(DEFAULT_FN)).flatten()?;
        let func = Ident::new(&fn_name, name.span());
        Some(quote! {
            builder.#name = builder.#name.or(Some(#func()));
        })
    });

    // 6. Generate the `build` method
    // 6a. TODO: Error out if any required fields are missing
    let create_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: builder.#name.unwrap_or_default()
        }
    });

    // 7. TODO: Save fields to data store
    let out = quote! {
        #[derive(Default)]
        pub struct #name {
            #(#optionized),*
        }

        impl appconfig_derive::AppConfig for #orig_name {}

        impl #orig_name {
            pub fn build(data_src: &mut impl appconfig_derive::DataSource) -> Result<Self, appconfig_derive::AppConfigError> {
                #(#assert_types)*
                let mut builder = #name::default();
                #(#read_from_data_src)*
                #(#read_from_env)*
                #(#read_from_default)*
                #(#read_from_default_fn)*
                Ok(Self {
                    #(#create_fields),*
                })
            }
        }
    };
    out.into()
}
