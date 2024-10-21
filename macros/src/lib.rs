use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// TODO: change this to a derive macro
#[proc_macro_attribute]
pub fn command_declaration(args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(item as syn::ItemStruct);
    let fields = &struct_item.fields;
    
    let struct_ident = &struct_item.ident;
    let struct_new_fields = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("struct field must have a name");
        let value_type = &field.ty;
        quote! {
            #field_ident: #value_type::parse(stream)?
        }
    });

    TokenStream::from(quote! {
        #struct_item

        impl command::parser::Parseable for #struct_ident {
            fn parse(stream: &mut command::parser::StringStream) -> command::parser::ParsingResult<Self>
            where
                Self: Sized,
            {
                let command_name = String::parse(stream);
                Ok(Self {
                    #(
                        #struct_new_fields,
                    )*
                })
            }
        }
    })
}

#[proc_macro_attribute]
pub fn subcommand(args: TokenStream, item: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(item as syn::ItemEnum);
    let enum_ident = &enum_item.ident;
    let variants = &enum_item.variants;

    let match_conditions = variants.iter().map(|variant| {
        let field_values = variant.fields.iter().map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let value_type = &field.ty;
            quote! {
                #field_ident: #value_type::parse(stream)?,
            }
        });
        let ident = &variant.ident;
        quote! {
            case "ident" => #enum_ident.#ident(
                #(
                    #field_values
                 )*
            ),
        }
    });

    TokenStream::from(quote! {
        #enum_item
    
        impl command::parser::Parseable for #enum_ident {
            fn parse(stream: &mut command::parser::StringStream) -> command::parser::ParsingResult<Self>
            where
                Self: Sized,
            {
                match stream.next().unwrap().as_str() {
                    #(#match_conditions)*
                    _ => Err("Subcommand not found")
                }
            }
        }
    })
}

#[proc_macro_attribute]
pub fn argument(args: TokenStream, item: TokenStream) -> TokenStream {
    item
}