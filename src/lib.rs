use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn elapsed(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let vis = &func.vis;
    let block = &func.block;
    let attrs = &func.attrs;

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
        #(#attrs),*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs) #output #where_clause {
            use std::time;

            let start = time::Instant::now();
            let fn_return_value = #block;
            println!("fn {} costs {}", #name_str, human_time::human_time(start.elapsed()));
            fn_return_value
        }
    };

    new_fn.into()
}
