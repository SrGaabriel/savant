use std::any::Any;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Type};

#[proc_macro_attribute]
pub fn command_declaration(args: TokenStream, item: TokenStream) -> TokenStream {
    println!("Opinha: {item}");
    let struct_item = parse_macro_input!(item as syn::ItemStruct);
    let fields = &struct_item.fields;
    let arguments = vec![];
    
    for field in fields {
        let argument_type = match &field.ty {
            Type::Path(type_path) => {
                let segments = &type_path.path.segments;
                if segments.len() == 1 {
                    match segments[0].ident.to_string().as_str() {
                        "bool" => 
                        "i32" => println!("It's an i32!"),
                        "String" => println!("It's a String!"),
                        _ => println!("It's some other type: {}", segments[0].ident),
                    }
                }
            }
            _ => panic!("Only simple types are supported")
        }
    }
    
    TokenStream::from(quote! {
        #struct_item
    })
}

#[proc_macro_attribute]
pub fn argument(args: TokenStream, item: TokenStream) -> TokenStream {
    item
}