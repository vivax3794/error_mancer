use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    self,
    parse2,
    parse_macro_input,
    parse_quote,
    GenericArgument,
    Path,
    PathArguments,
    ReturnType,
    Token,
    Type,
    TypePath,
};

#[proc_macro_attribute]
pub fn errors(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr);
    let item = parse_macro_input!(item);
    match errors_impl(attr, item) {
        Ok(result) => result.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn errors_impl(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    if let Ok(function) = syn::parse2(item.clone()) {
        do_free_function(function, attr)
    } else if let Ok(impl_block) = syn::parse2(item.clone()) {
        do_impl_block(impl_block)
    } else {
        Err(syn::Error::new(
            item.span(),
            "Expected function or impl block",
        ))
    }
}

fn do_impl_block(mut impl_block: syn::ItemImpl) -> syn::Result<TokenStream> {
    let mut enums = Vec::new();
    for item in &mut impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            if let Some(attr) = method
                .attrs
                .iter()
                .find(|&attr| attr.path().is_ident("errors"))
            {
                match attr.meta.clone() {
                    syn::Meta::List(list) => {
                        let arguments = list.tokens;
                        let function = method.into_token_stream();
                        let function = parse2(function)?;
                        let (enum_decl, function) = create_function(function, arguments)?;
                        enums.push(enum_decl);

                        let function = function.into_token_stream();
                        let function = parse2(function)?;
                        *method = function;
                    }
                    syn::Meta::Path(_) => {
                        let arguments = quote!();
                        let function = method.into_token_stream();
                        let function = parse2(function)?;
                        let (enum_decl, function) = create_function(function, arguments)?;
                        enums.push(enum_decl);

                        let function = function.into_token_stream();
                        let function = parse2(function)?;
                        *method = function;
                    }
                    _ => {
                        return Err(syn::Error::new(
                            attr.span(),
                            "Expected list or simple `#[errors]`",
                        ))
                    }
                }
            }
        }
    }

    Ok(quote! {
        #(#enums)*
        #impl_block
    })
}

fn do_free_function(function: syn::ItemFn, attr: TokenStream) -> Result<TokenStream, syn::Error> {
    let (enum_decl, new_function) = create_function(function, attr)?;
    Ok(quote! {
        #enum_decl
        #new_function
    })
}

fn create_function(
    function: syn::ItemFn,
    attr: TokenStream,
) -> Result<(TokenStream, TokenStream), syn::Error> {
    let derives = match function
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("derive"))
    {
        Some(attr) => attr.to_token_stream(),
        None => quote!(),
    };

    let vis = function.vis;
    let mut signature = function.sig;
    let body = function.block;

    let (ok_return_type, explicit_error_name) = get_return_generics(&signature.output)?;
    let (error_enum, error_return_type) = generate_error_type(
        attr,
        signature.ident.to_string(),
        vis.clone(),
        derives,
        explicit_error_name.clone(),
    )?;

    let inner_type: syn::ReturnType =
        parse_quote!(-> ::core::result::Result<#ok_return_type, #error_return_type>);

    let replaced = replace_error_value(&mut signature.output, error_return_type);

    let emit_enum_outside = replaced || explicit_error_name.is_some();

    if emit_enum_outside {
        let new_func = quote! {
            #[allow(clippy::needless_question_mark)]
            #vis #signature {
                Ok((move || #inner_type { #body })()?)
            }
        };
        Ok((error_enum, new_func))
    } else {
        let new_func = quote! {
            #[allow(clippy::needless_question_mark)]
            #vis #signature {
                #error_enum
                Ok((move || #inner_type { #body })()?)
            }
        };
        Ok((quote!(), new_func))
    }
}

fn get_return_generics(return_type: &ReturnType) -> syn::Result<(&Type, Option<syn::Ident>)> {
    match return_type {
        ReturnType::Default => Err(syn::Error::new(
            return_type.span(),
            "Function must have a return type of Result<Ok, Err>",
        )),
        ReturnType::Type(_, ty) => {
            // Ensure the return type is a Path type
            let type_path = match ty.as_ref() {
                Type::Path(TypePath { path, .. }) => path,
                _ => {
                    return Err(syn::Error::new(
                        ty.span(),
                        "Expected return type to be a path, such as Result<Ok, Err>",
                    ))
                }
            };

            // Check if the last segment is 'Result'
            let last_segment = type_path.segments.last().ok_or_else(|| {
                syn::Error::new(type_path.span(), "Expected a path segment for Result")
            })?;

            if last_segment.ident != "Result" {
                return Err(syn::Error::new(
                    last_segment.ident.span(),
                    "Expected return type to be Result<...>",
                ));
            }

            // Ensure that Result has exactly two generic arguments
            let generic_args = match &last_segment.arguments {
                PathArguments::AngleBracketed(args) => &args.args,
                _ => {
                    return Err(syn::Error::new(
                        last_segment.span(),
                        "Expected angle-bracketed generic arguments, like Result<Ok, Err>",
                    ))
                }
            };

            // Extract the first generic argument (Ok type)
            let ok_arg = generic_args.first().ok_or_else(|| {
                syn::Error::new(
                    generic_args.span(),
                    "Expected at least one generic argument for Result",
                )
            })?;
            let ok_type = match ok_arg {
                GenericArgument::Type(ok_type) => ok_type,
                _ => {
                    return Err(syn::Error::new(
                        ok_arg.span(),
                        "Expected the first generic argument of Result to be a type",
                    ))
                }
            };

            // Extract the second generic argument (Ok type)
            let err_arg = generic_args.get(1);
            let enum_name = match err_arg {
                Some(GenericArgument::Type(Type::Infer(_))) => None,
                Some(GenericArgument::Type(Type::Path(TypePath {
                    path: Path { segments, .. },
                    qself: None,
                }))) => {
                    if segments.len() != 1 {
                        return Ok((ok_type, None));
                    }
                    let segment = &segments[0];

                    if !segment.arguments.is_empty() {
                        return Ok((ok_type, None));
                    }

                    Some(segment.ident.clone())
                }
                _ => None,
            };

            Ok((ok_type, enum_name))
        }
    }
}

fn replace_error_value(return_type: &mut ReturnType, error_type: syn::Type) -> bool {
    let ReturnType::Type(_, return_type) = return_type else {
        return false;
    };

    let syn::Type::Path(return_type) = return_type.as_mut() else {
        return false;
    };

    let Some(last) = return_type.path.segments.last_mut() else {
        return false;
    };

    if last.ident != "Result" {
        return false;
    }

    let syn::PathArguments::AngleBracketed(arguments) = &mut last.arguments else {
        return false;
    };

    if arguments.args.len() < 2 {
        return false;
    }

    if let syn::GenericArgument::Type(syn::Type::Infer(_)) = arguments.args[1] {
        arguments.args[1] = syn::GenericArgument::Type(error_type);
        return true;
    }
    false
}

fn generate_error_type(
    args: TokenStream,
    function_name: String,
    vis: syn::Visibility,
    derives: TokenStream,
    enum_name: Option<syn::Ident>,
) -> syn::Result<(TokenStream, Type)> {
    let enum_name = if let Some(enum_name) = enum_name {
        enum_name
    } else {
        let enum_name = function_name.to_case(Case::Pascal);
        format_ident!("{enum_name}Error")
    };

    let error_types = Punctuated::<syn::Path, Token![,]>::parse_terminated.parse2(args)?;
    let error_types_clone = error_types.clone().into_iter().collect::<Vec<_>>();
    let (fields, from_impls): (Vec<_>, Vec<_>) = error_types
        .iter()
        .map(|path| {
            let name = path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string() + "_")
                .collect::<String>()
                .to_case(Case::Pascal);
            let name = name.trim_end_matches("Error");
            let name = format_ident!("{name}");

            (
                (
                    name.clone(),
                    quote!(
                        #name(#path)
                    ),
                ),
                quote!(
                    impl ::error_mancer::ErrorMancerFrom<#path> for #enum_name {
                        fn from(value: #path) -> Self {
                            Self::#name(value)
                        }
                    }
                ),
            )
        })
        .unzip();
    let (names, fields): (Vec<_>, Vec<_>) = fields.into_iter().unzip();

    let enum_stream = quote! {
        #[derive(::core::fmt::Debug)]
        #derives
        #vis enum #enum_name {
            #(#fields),*
        }

        #(#from_impls)*

        impl<T> ::core::convert::From<T> for #enum_name where Self: ::error_mancer::ErrorMancerFrom<T> {
            fn from(value: T) -> Self {
                ::error_mancer::ErrorMancerFrom::from(value)
            }
        }

        impl<T> ::error_mancer::FlattenInto<T> for #enum_name
            where T: #(::error_mancer::ErrorMancerFrom<#error_types_clone>)+* {
            fn flatten(self) -> T {
                match self {
                    #(Self::#names(err) => T::from(err),)*
                    _ => unreachable!()
                }
            }
        }

        impl ::core::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    #(Self::#names(err) => err.fmt(f),)*
                    _ => unreachable!()
                }
            }
        }

        impl ::core::error::Error for #enum_name {}
    };
    let enum_type = parse_quote!(#enum_name);

    Ok((enum_stream, enum_type))
}
