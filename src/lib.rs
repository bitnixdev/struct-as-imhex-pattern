extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(AsImHexPattern)]
pub fn struct_to_string(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let name = ast.ident;

    let mut fields = String::new();

    if let syn::Data::Struct(data_struct) = ast.data {
        for field in data_struct.fields {
            let field_name = field.ident;
            let field_type = field.ty;

            fields.push_str(&format!(
                "  {};\n",
                as_pattern_type(&field_name, &field_type),
            ));
        }
    }

    let gen = quote! {
        impl #name {
            pub fn as_imhex_pattern() -> String {
                let mut res = String::from("");
                res.push_str("struct ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#fields);
                res.push_str("};\n");
                res
            }

            pub fn as_imhex_pattern_root() -> String {
                let mut res = String::from("");
                res.push_str("struct ");
                res.push_str(stringify!(#name));
                res.push_str(" {\n");
                res.push_str(#fields);
                res.push_str("};\n");
                res.push_str(stringify!(#name));
                res.push_str(" ");
                res.push_str(stringify!(#name));
                res.push_str(" @ 0x0;\n");
                res
            }
        }
    };

    gen.into()
}

fn as_pattern_type(field_name: &Option<Ident>, r#type: &Type) -> String {
    match r#type {
        Type::Path(type_path) => {
            let final_type = type_path.path.segments.last().unwrap().ident.to_string();
            let final_type = match final_type.as_str() {
                "f32" => "float",
                "f64" => "double",
                _ => &final_type,
            }
            .to_string();
            if let Some(field_name) = field_name {
                if field_name.to_string().starts_with("_") {
                    format!("{}", final_type)
                } else {
                    format!("{} {}", final_type, field_name)
                }
            } else {
                format!("{}", final_type)
            }
        }
        Type::Array(array) => {
            let inner_type = as_pattern_type(&None, &array.elem);
            let mut digits = "".to_string();
            if let syn::Expr::Lit(x) = &array.len {
                if let syn::Lit::Int(x) = &x.lit {
                    digits = x.base10_digits().to_string();
                }
            };
            if let Some(field_name) = field_name {
                if field_name.to_string().starts_with("_") {
                    format!("padding[{}]", digits)
                } else {
                    format!("{} {}[{}]", inner_type, field_name, digits)
                }
            } else {
                format!("padding[{}]", digits)
            }
        }
        _ => {
            if let Some(field_name) = field_name {
                if field_name.to_string().starts_with("_") {
                    format!("{}", field_name)
                } else {
                    format!("{} {}", field_name, field_name)
                }
            } else {
                "".to_string()
            }
        }
    }
}
