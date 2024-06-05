//! A procedural macro library that enables structs with the [`RowConsumer`] trait to
//! provide a `from_row` implementation. Please refer to pgde for use, examples, and
//! limitations.
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

/// A macro for deriving a `from_row` implementation onto a struct.
#[proc_macro_derive(RowConsumer)]
pub fn derive_row_consumer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let data = input.data;
    parse_field_setters(&name, &data)
}

/// A function that takes a given [`TokenStream`]'s [`Ident`] and [`Data`] and returns a
/// [`TokenStream`] for implementing a `from_row` from a struct's fields.
fn parse_field_setters(class_name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_setters = fields.named.iter().enumerate().map(|(i, f)| {
                    let field_name = &f.ident;
                    let field_type = &f.ty;

                    quote! {
                        #field_name: match row.try_get::<usize, #field_type>(#i) {
                            Ok(v) => v,
                            Err(_) => {
                                errors.push(format!("Conversion error occurred for field \"{}\" on class \"{}\"", stringify!(#field_name), stringify!(#class_name)));
                                #field_type::default()
                            },
                        }
                    }
                });

                let implementation = quote! {
                    impl pgde::RowConsumer for #class_name {
                        fn from_row(row: Row) -> Result<Self, (Self, Vec<String>)>
                        where
                            Self: Sized,
                        {
                            let mut errors : Vec<String> = Vec::new();

                            let class_instance = Self {
                                #(#field_setters),*
                            };

                            match errors.len() {
                                0 => Ok(class_instance),
                                _ => Err((class_instance, errors)),
                            }
                        }
                    }
                };

                TokenStream::from(implementation)
            }
            Fields::Unnamed(_) | Fields::Unit => panic!(
                "RowConsumer is not supported on unit structs nor structs with unnamed fieds"
            ),
        },
        Data::Enum(_) | Data::Union(_) => panic!("RowConsumer is not supported on enums or unions"),
    }
}
