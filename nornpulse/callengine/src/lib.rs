use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parenthesized, parse_macro_input, Attribute, FnArg, Ident, LitInt, LitStr, ReturnType, Token, Visibility,
};

struct EngineSig {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    inputs: Punctuated<FnArg, Comma>,
    output: ReturnType,
}

struct CallInfo {
    address: LitInt,
    calling_convention: Option<LitStr>,
}

impl Parse for EngineSig {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse attributes.
        let attrs = input.call(Attribute::parse_outer)?;

        // Parse function visibility
        let vis: syn::Visibility = input.parse()?;

        input.parse::<Token![unsafe]>()?;
        input.parse::<Token![fn]>()?;

        // Parse function name.
        let ident: Ident = input.parse()?;

        // Parse function inputs.
        let content;
        let _ = parenthesized!(content in input);
        let inputs = Punctuated::<FnArg, Comma>::parse_terminated(&content)?;

        // Parse function output.
        let output: syn::ReturnType = input.parse()?;

        input.parse::<Token![;]>()?;

        Ok(EngineSig {
            attrs: attrs,
            vis: vis,
            ident: ident,
            inputs: inputs,
            output: output,
        })
    }
}

impl EngineSig {
    pub fn is_method(&self) -> bool {
        let arg = self.inputs.iter().next();
        match arg {
            Some(FnArg::Receiver(_)) => true,
            _ => false,
        }
    }
}

impl Parse for CallInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let address: LitInt = input.parse()?;

        let calling_convention = input
            .parse::<Token![,]>()
            .and_then(|_| input.parse::<LitStr>())
            .ok();

        Ok(CallInfo {
            address: address,
            calling_convention: calling_convention,
        })
    }
}

/// Generates the calling arguments for the ABI function. Does this by getting
/// the names from the list of name/type pairs of function arguments. For
/// purposes of ABI interop we wrap self in `std::mem::transmute`.
fn get_names(input: &Punctuated<FnArg, Comma>) -> TokenStream2 {
    let args = input.iter().map(|s| match s {
        FnArg::Receiver(_) => "std::mem::transmute(self)".parse().unwrap(),
        FnArg::Typed(t) => t.pat.to_token_stream(),
    });
    quote! {#(#args),*}
}

/// Given the function's name/type pairs, returns the types, with `&self`
/// transformed into a `*mut usize` as the ABI function can't use the `Self`
/// type.
fn ffi_types(input: &Punctuated<FnArg, Comma>) -> TokenStream2 {
    let args = input.iter().map(|s| match s {
        FnArg::Receiver(_) => "*mut usize".parse().unwrap(),
        FnArg::Typed(t) => t.ty.to_token_stream(),
    });
    quote! {#(#args),*}
}

#[proc_macro_attribute]
pub fn call_engine(address_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let engine_sig = parse_macro_input!(item);
    let EngineSig {
        attrs,
        vis,
        ident,
        inputs,
        output,
    } = &engine_sig;

    let CallInfo {
        address,
        calling_convention,
    } = parse_macro_input!(address_attrs);

    let arg_names = get_names(&inputs);
    let arg_types = ffi_types(&inputs);

    let conventions = if calling_convention.is_none() {
        if engine_sig.is_method() {
            "\"thiscall\""
        } else {
            "\"stdcall\""
        }
        .parse()
        .unwrap()
    } else {
        calling_convention.unwrap().to_token_stream()
    };

    let expanded = quote! {
      #(#attrs)
* 
      #vis unsafe fn #ident(#inputs) #output
      {
        type F = unsafe extern #conventions fn(#arg_types) #output;
        let op: F = std::mem::transmute(#address as usize);
        op(#arg_names)
      }
    };
    expanded.into()
}
