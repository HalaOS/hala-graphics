use proc_macro::TokenStream;

use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Expr, ExprLit, Field, Ident, ItemStruct, Lit, Type};

fn invalid_field<E: AsRef<str>>(index: usize, field: &Field, error: E) -> ! {
    panic!(
        "Invalid field({}) type: {}.",
        field
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or(index.to_string()),
        error.as_ref()
    );
}

#[allow(unused)]
struct VertexFormat {
    offset: usize,
    bytes_len: usize,
    element_len: usize,
    format: Ident,
    field_ident: proc_macro2::TokenStream,
}

fn get_array_len(expr: &Expr) -> Option<usize> {
    if let Expr::Lit(ExprLit { attrs: _, lit }) = expr {
        if let Lit::Int(value) = lit {
            return value.base10_parse::<usize>().ok();
        }
    };

    None
}

fn field_to_vertex_format(index: usize, field: &Field) -> VertexFormat {
    let normalize_ident = format_ident!("normalize");

    let is_normalize = field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(&normalize_ident));

    let field_ident = field
        .ident
        .clone()
        .map(|indent| indent.to_token_stream())
        .unwrap_or(quote!(#index));

    if let Type::Array(array) = &field.ty {
        if let Type::Path(path) = array.elem.as_ref() {
            let len = match get_array_len(&array.len) {
                Some(len) => len,
                None => {
                    invalid_field(index, field, "invalid array length");
                }
            };

            match path.to_token_stream().to_string().as_str() {
                "u8" => {
                    if len != 2 && len != 4 {
                        invalid_field(
                            index,
                            field,
                            "valid format: uint8x2, uint8x4, unorm8x2, unorm8x4",
                        );
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len,
                        element_len: len,
                        format: if is_normalize {
                            format_ident!("Unorm8x{}", len)
                        } else {
                            format_ident!("Uint8x{}", len)
                        },
                        field_ident,
                    };
                }
                "i8" => {
                    if len != 2 && len != 4 {
                        invalid_field(
                            index,
                            field,
                            "valid format: sint8x2, sint8x4, snorm8x2, snorm8x4",
                        );
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len,
                        element_len: len,
                        format: if is_normalize {
                            format_ident!("Snorm8x{}", len)
                        } else {
                            format_ident!("Sint8x{}", len)
                        },
                        field_ident,
                    };
                }
                "u16" => {
                    if len != 2 && len != 4 {
                        invalid_field(
                            index,
                            field,
                            "valid format: uint16x2, uint16x4, unorm16x2, unorm16x4",
                        );
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 2,
                        element_len: len,
                        format: if is_normalize {
                            format_ident!("Unorm16x{}", len)
                        } else {
                            format_ident!("Uint16x{}", len)
                        },
                        field_ident,
                    };
                }
                "i16" => {
                    if len != 2 && len != 4 {
                        invalid_field(
                            index,
                            field,
                            "valid format: sint16x2, sint16x4, snorm16x2, snorm16x4",
                        );
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 2,
                        element_len: len,
                        format: if is_normalize {
                            format_ident!("Snorm16x{}", len)
                        } else {
                            format_ident!("Sint16x{}", len)
                        },
                        field_ident,
                    };
                }
                "f16" => {
                    if len != 2 && len != 4 {
                        invalid_field(index, field, "valid format: float16x2, float16x4");
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 2,
                        element_len: len,
                        format: format_ident!("Float16x{}", len),
                        field_ident,
                    };
                }
                "f32" => {
                    if len != 2 && len != 3 && len != 4 {
                        invalid_field(
                            index,
                            field,
                            "valid format: float32x2, float32x3, float32x4",
                        );
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 4,
                        element_len: len,
                        format: format_ident!("Float32x{}", len),
                        field_ident,
                    };
                }
                "i32" => {
                    if len != 2 && len != 3 && len != 4 {
                        invalid_field(index, field, "valid format: sint32x2, sint32x3, sint32x4");
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 4,
                        element_len: len,
                        format: format_ident!("Sint32x{}", len),
                        field_ident,
                    };
                }
                "u32" => {
                    if len != 2 && len != 3 && len != 4 {
                        invalid_field(index, field, "valid format: uint32x2, uint32x3, uint32x4");
                    }

                    return VertexFormat {
                        offset: index,
                        bytes_len: len * 4,
                        element_len: len,
                        format: format_ident!("Uint32x{}", len),
                        field_ident,
                    };
                }
                _ => {}
            }
        }
    }

    invalid_field(index, field, "use 'skip' to ignore this field")
}

#[proc_macro_derive(Vertex, attributes(skip, normalize))]
pub fn drive_wgpu_vertex(item: TokenStream) -> TokenStream {
    let skip_ident = format_ident!("skip");

    let input = parse_macro_input!(item as ItemStruct);

    let name = input.ident;

    let fields = input
        .fields
        .into_iter()
        .enumerate()
        .filter_map(|(index, field)| {
            if field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident(&skip_ident))
            {
                return None;
            }

            Some(field_to_vertex_format(index, &field))
        })
        .collect::<Vec<_>>();

    let array_stride = fields
        .iter()
        .fold(0usize, |sum, format| sum + format.bytes_len);

    let length = fields.len();

    let attrs = fields
        .iter()
        .enumerate()
        .map(|(index, format)| {
            let format = &format.format;

            let token_stream: proc_macro2::TokenStream =
                format!("{} => {}", index, format).parse().unwrap();

            token_stream
        })
        .collect::<Vec<_>>();

    let mut to_bytes = vec![];

    let mut offset = 0usize;

    for format in fields {
        let ident = format.field_ident;

        let end = offset + format.bytes_len;

        to_bytes.push(quote! {
            buf[#offset..#end].copy_from_slice(bytemuck::cast_slice(&self.#ident));
        });

        offset += format.bytes_len;
    }

    quote! {
        impl #name {
            fn vertex_buff_layout() -> wgpu::VertexBufferLayout<'static> {
                use std::mem;

                const ATTRIBS: [wgpu::VertexAttribute; #length] = wgpu::vertex_attr_array![#(#attrs,)*];

                wgpu::VertexBufferLayout {
                    array_stride: #array_stride as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRIBS,
                }
            }

            fn to_bytes<Buf>(&self, mut buf: Buf) where Buf: AsMut<[u8]>{
                let buf = buf.as_mut();

                assert!(buf.len() > #array_stride, "bo_bytes: output buf too short");
                #(#to_bytes)*
            }
        }
    }.into()
}
