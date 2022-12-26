use std::collections::BTreeMap;

use proc_macro::TokenStream;
use quote::{__private::Span, quote};
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, Error, Ident, Lit, Meta,
    MetaNameValue, Token,
};
use syn::{parse::ParseStream, ItemFn};
use syn::{parse::Result, NestedMeta};

#[derive(Debug)]
struct Args {
    attributes: BTreeMap<String, String>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        try_parse(input)
    }
}

fn try_parse(input: ParseStream) -> Result<Args> {
    let args = Punctuated::<NestedMeta, Token![,]>::parse_terminated(input)?;

    let attributes = args
        .iter()
        .map(|arg| match arg {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(lit_str),
                path,
                ..
            })) => (
                path.segments.first().unwrap().ident.to_string(),
                lit_str.value(),
            ),
            _ => panic!("illegal arguments, example: output=\"eprintln\""),
        })
        .collect::<BTreeMap<_, _>>();

    Ok(Args { attributes })
}

fn _error(msg: &str) -> Error {
    Error::new(Span::call_site(), msg)
}

#[proc_macro_attribute]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    let args: Args = parse_macro_input!(args as Args);

    let output_target = match args.attributes.get("output") {
        Some(s) => s.clone(),
        None => "println".to_string(),
    };

    let output_target = Ident::new(output_target.as_str(), Span::call_site());

    let func = parse_macro_input!(func as ItemFn);
    let vis = &func.vis;
    let block = &func.block;

    let signature = func.sig;
    let asyncness = &signature.asyncness;
    let constness = &signature.constness;
    let unsafety = &signature.unsafety;
    let abi = &signature.abi;
    let ident = &signature.ident;
    let name_str = ident.to_string();
    let generics = &signature.generics;
    let inputs = &signature.inputs;
    let output = &signature.output;
    let where_clause = &signature.generics.where_clause;

    let new_fn = quote! {

        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs) #output #where_clause {
            use std::time;

            let start = time::Instant::now();
            let fn_return_value = #block;
            #output_target!("fn {} costs {}", #name_str, human_time::human_time(start.elapsed()));
            fn_return_value
        }
    };

    new_fn.into()
}
