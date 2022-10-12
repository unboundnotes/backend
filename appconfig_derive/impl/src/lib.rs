use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(AppConfig, attributes(config))]
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

    // 1. Generate a `builder` struct, with all optional fields
    // TODO: Treat `Vec`, `HashSet`, `Array`, `Option` and `AppConfig` fields differently
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            #name: Option<#ty>
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
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            match data_src.get(stringify!(#name)) {
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
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            builder.#name = builder.#name.or(
                std::env::var(stringify!(#name))
                    .ok()
                    .map(|value| value.parse::<#ty>().map_err(|e| appconfig_derive::AppConfigError::ParsingError(Box::new(e))))
                    .transpose()?
            );
        }
    });

    // 5. TODO: Try to load the values from the default option
    // 5a. TODO: This should also allow default_fn to be specified
    // let read_from_default = fields.iter().map(|f| {
    //     let name = &f.ident;
    //     match f.attrs.iter().find(|a| a.path.is_ident("config")) {
    //         None => quote! {},
    //         Some(_attrs) => {
    //             // eprintln!("{:#?}", attrs.parse_args::<syn::Expr>());
    //             quote! {
    //                 builder.#name = builder.#name;
    //             }
    //         }
    //     }
    // });

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
                // #(#read_from_default)*
                Ok(Self {
                    #(#create_fields),*
                })
            }
        }
    };
    out.into()
}
