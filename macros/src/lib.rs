use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Type};
use command::{Argument, ArgumentType};

// TODO: change this to a derive macro
#[proc_macro_attribute]
pub fn command_declaration(args: TokenStream, item: TokenStream) -> TokenStream {
    let struct_item = parse_macro_input!(item as syn::ItemStruct);
    let fields = &struct_item.fields;
    let mut arguments = vec![];
    
    for field in fields {
        let argument_type = match &field.ty {
            Type::Path(type_path) => {
                let segments = &type_path.path.segments;
                if segments.len() == 1 {
                    match segments[0].ident.to_string().as_str() {
                        "bool" => ArgumentType::Boolean,
                        "i32" => ArgumentType::Int,
                        "String" => ArgumentType::String,
                        _ => panic!("Unsupported argument type: {}", segments[0].ident),
                    }
                } else {
                    panic!("Please use simple types (`String`, `bool`, etc.)");
                }
            }
            _ => panic!("Only simple types are supported")
        };
        arguments.push(Argument::new(field.ident.as_ref().unwrap().to_string(), argument_type));
    }
    let struct_ident = &struct_item.ident;
    let struct_new_fields = arguments.iter().map(|argument| {
        let name = syn::parse_str::<syn::Ident>(&argument.long).unwrap();
        let value_type = syn::parse_str::<Type>(&argument.value_type.name()).unwrap();
        quote! {
            #name: #value_type::parse(stream)?
        }
    });

    TokenStream::from(quote! {
        #struct_item

        impl command::parser::Parseable for #struct_ident {
            fn parse(stream: &mut command::parser::StringStream) -> command::parser::ParsingResult<Self>
            where
                Self: Sized,
            {
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
pub fn argument(args: TokenStream, item: TokenStream) -> TokenStream {
    item
}