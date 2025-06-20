use proc_macro::TokenStream;
use quote::quote;
use syn::{Type, parse_macro_input};

fn is_u16(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("u16"))
}

fn is_i64(ty: &Type) -> bool {
    matches!(ty, Type::Path(type_path) if type_path.path.is_ident("i64"))
}

#[proc_macro_derive(BinCodec)]
pub fn derive_binary_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let syn::Data::Struct(data_struct) = &input.data else {
        panic!("BinCodec can only be derived for structs");
    };

    let mut encode_fields = Vec::new();
    let mut decode_fields = Vec::new();
    let mut construct_fields = Vec::new();

    for field in &data_struct.fields {
        let field_name = field.ident.as_ref().expect("Fields must be named");
        let field_type = &field.ty;

        let encode = if is_u16(field_type) {
            quote! {
                buf.put_u16(self.#field_name);
            }
        } else if is_i64(field_type) {
            quote! {
                buf.put_i64(self.#field_name);
            }
        } else {
            quote! {
                <#field_type as BinaryCodec>::encode(&self.#field_name, buf);
            }
        };
        encode_fields.push(encode);

        let decode = if is_u16(field_type) {
            quote! {
                let #field_name = buf.get_u16();
            }
        } else if is_i64(field_type) {
            quote! {
                let #field_name = buf.get_i64();
            }
        } else {
            quote! {
                let #field_name = <#field_type as BinaryCodec>::decode(buf).ok_or("Failed to decode field")?;
            }
        };
        decode_fields.push(decode);

        construct_fields.push(quote! {
            #field_name: #field_name,
        });
    }

    let expanded = quote! {
        impl BinaryCodec for #name {
            fn encode(&self, buf: &mut bytes::BytesMut) {
                use bytes::BufMut;
                #(#encode_fields)*
            }

            fn decode(buf: &mut bytes::Bytes) -> Option<Self> {
                use bytes::Buf;
                #(#decode_fields)*
                Some(Self {
                    #(#construct_fields)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
