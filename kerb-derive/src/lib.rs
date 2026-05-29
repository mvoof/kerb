use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// Derive `to_snapshot()` for a `#[repr(C, packed)]` struct.
///
/// By default references `crate::TelemetryValue`. When used from an external
/// crate, override with `#[snapshot(crate = "kerb")]`.
///
/// - Scalar fields are inserted directly.
/// - `[u8; N]` fields are decoded as null-terminated ASCII strings.
/// - `[u16; N]` fields are decoded as null-terminated UTF-16 strings.
/// - `[T; N]` scalar arrays are inserted as `field_0 .. field_N-1`.
/// - `[[T; N]; M]` 2D arrays are skipped.
/// - Fields starting with `_` (padding/expansion) are skipped.
/// - Nested structs with `to_snapshot()` are merged with a `field.` prefix.
#[proc_macro_derive(Snapshot, attributes(snapshot))]
pub fn derive_snapshot(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let krate: TokenStream2 = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("snapshot"))
        .and_then(|a| {
            a.parse_args::<syn::MetaNameValue>().ok().and_then(|nv| {
                if nv.path.is_ident("crate")
                    && let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = nv.value
                {
                    let path: syn::Path = s.parse().ok()?;
                    return Some(quote! { #path });
                }
                None
            })
        })
        .unwrap_or_else(|| quote! { crate });

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("Snapshot only supports named-field structs"),
        },
        _ => panic!("Snapshot only supports structs"),
    };

    let mut inserts = Vec::<TokenStream2>::new();

    for field in fields {
        let fname = field.ident.as_ref().unwrap();
        let fname_str = fname.to_string();

        if fname_str.starts_with('_') {
            continue;
        }

        let ty = &field.ty;

        if let Some((elem_ty, len)) = array_type(ty) {
            if is_u8(&elem_ty) {
                // [u8; N] — null-terminated ASCII string
                inserts.push(quote! {
                    {
                        let bytes = self.#fname;
                        let s: String = bytes.iter()
                            .take_while(|&&b| b != 0)
                            .map(|&b| b as char)
                            .collect();
                        m.insert(#fname_str.to_string(), #krate::TelemetryValue::Text(s));
                    }
                });
            } else if is_u16(&elem_ty) {
                // [u16; N] — null-terminated UTF-16 string
                inserts.push(quote! {
                    {
                        let arr = self.#fname;
                        let end = arr.iter().position(|&x| x == 0).unwrap_or(#len);
                        let s = String::from_utf16_lossy(&arr[..end]).to_string();
                        m.insert(#fname_str.to_string(), #krate::TelemetryValue::Text(s));
                    }
                });
            } else if is_scalar(&elem_ty) {
                // [T; N] — insert as field_0, field_1, ...
                for i in 0..len {
                    let key = format!("{}_{}", fname_str, i);
                    inserts.push(quote! {
                        {
                            let v = self.#fname[#i];
                            m.insert(#key.to_string(), #krate::TelemetryValue::from(v));
                        }
                    });
                }
            }
            // [[T; N]; M] 2D arrays and other complex types — skip
        } else if is_scalar(ty) {
            inserts.push(quote! {
                {
                    let v = self.#fname;
                    m.insert(#fname_str.to_string(), #krate::TelemetryValue::from(v));
                }
            });
        } else if is_nested_struct(ty) {
            // Nested struct with its own to_snapshot() — merge with prefix
            inserts.push(quote! {
                {
                    let nested = self.#fname.to_snapshot();
                    for (k, v) in nested {
                        m.insert(format!("{}.{}", #fname_str, k), v);
                    }
                }
            });
        }
        // 2D arrays, arrays of structs, and other unrecognized types are skipped
    }

    quote! {
        impl #name {
            pub fn to_snapshot(&self) -> std::collections::HashMap<String, #krate::TelemetryValue> {
                let mut m = std::collections::HashMap::new();
                #(#inserts)*
                m
            }
        }
    }
    .into()
}

fn array_type(ty: &Type) -> Option<(Type, usize)> {
    if let Type::Array(arr) = ty {
        // Skip 2D arrays [[T; N]; M]
        if matches!(*arr.elem, Type::Array(_)) {
            return None;
        }
        if let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(ref lit_int),
            ..
        }) = arr.len
        {
            let len: usize = lit_int.base10_parse().ok()?;
            return Some((*arr.elem.clone(), len));
        }
    }
    None
}

fn is_nested_struct(ty: &Type) -> bool {
    if let Type::Path(p) = ty
        && let Some(ident) = p.path.get_ident()
    {
        // Not a primitive scalar
        return !matches!(
            ident.to_string().as_str(),
            "f32"
                | "f64"
                | "i32"
                | "i64"
                | "u32"
                | "u64"
                | "u8"
                | "i8"
                | "u16"
                | "i16"
                | "bool"
                | "usize"
                | "isize"
        );
    }
    false
}

fn is_u8(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.is_ident("u8"))
}

fn is_u16(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.is_ident("u16"))
}

fn is_scalar(ty: &Type) -> bool {
    if let Type::Path(p) = ty
        && let Some(ident) = p.path.get_ident()
    {
        return matches!(
            ident.to_string().as_str(),
            "f32" | "f64" | "i32" | "i64" | "u32" | "u64" | "u8" | "i8" | "u16" | "i16" | "bool"
        );
    }
    false
}
