use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Meta, NestedMeta, Path};

pub fn derive(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let (impl_gen, type_gen, where_clause) = input.generics.split_for_impl();

    let mut sys_ty = None;
    let mut build_fn = None;

    for attr in input.attrs {
        if !attr.path.is_ident("system_desc") {
            continue;
        }

        let parsed = match parse_attribute(attr) {
            Ok(parsed) => parsed,
            Err(err) => {
                return err.to_compile_error();
            }
        };

        if let Some(ty) = parsed.0 {
            if let Some(old_ty) = sys_ty.replace(ty) {
                return Error::new_spanned(
                    old_ty,
                    "System type cannot be specified multiple times",
                )
                .to_compile_error();
            }
        }

        if let Some(ty) = parsed.1 {
            if let Some(old_ty) = build_fn.replace(ty) {
                return Error::new_spanned(
                    old_ty,
                    "Build function cannot be specified multiple times",
                )
                .to_compile_error();
            }
        }
    }

    let sys_ty = match sys_ty {
        None => {
            return Error::new(
                name.span(),
                "System type for `SystemDesc` must be specified with `system_desc` attribute",
            )
            .to_compile_error();
        }
        Some(x) => x,
    };

    let build_fn = match build_fn {
        None => {
            return Error::new(
                name.span(),
                "Build function for `SystemDesc` must be specified with `system_desc` attribute",
            )
            .to_compile_error();
        }
        Some(x) => x,
    };

    quote! {
        impl #impl_gen SystemDesc for #name #type_gen #where_clause {
            type SystemType = #sys_ty;

            fn build(self, world: &mut World) -> <Self::SystemType as SystemType>::System {
                #build_fn(world)
            }
        }
    }
}

fn parse_attribute(attr: Attribute) -> syn::Result<(Option<Path>, Option<Path>)> {
    let mut sys_ty = None;
    let mut build_fn = None;

    let list = if let Meta::List(list) = attr.parse_meta()? {
        list.nested
    } else {
        return Err(Error::new_spanned(
            attr.tokens,
            "The attribute must contain a list of `type` and `fn`",
        ));
    };

    for nested in list {
        match nested {
            NestedMeta::Meta(Meta::List(l)) => {
                if l.path.is_ident("type") || l.path.is_ident("fn") {
                    if l.nested.len() != 1 {
                        return Err(Error::new_spanned(
                            l.nested,
                            "There should be exactly one path in this list",
                        ));
                    }
                    let path = match l.nested.first().unwrap() {
                        NestedMeta::Meta(Meta::Path(path)) => path,
                        _ => {
                            return Err(Error::new_spanned(
                                l.nested,
                                "The given value should be a path",
                            ));
                        }
                    };
                    if l.path.is_ident("type") {
                        sys_ty.replace(path.clone());
                    } else {
                        build_fn.replace(path.clone());
                    }
                } else {
                    return Err(Error::new_spanned(
                        l,
                        "Either `type` or `fn` should be specified here",
                    ));
                }
            }
            _ => {
                return Err(Error::new_spanned(
                    nested,
                    "Either `type` or `fn` should be specified here",
                ));
            }
        }
    }

    Ok((sys_ty, build_fn))
}
