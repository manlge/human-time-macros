use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn elapsed(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let fn_vis = &func.vis;
    let fn_block = &func.block;

    let fn_sig = func.sig;
    let fn_ident = &fn_sig.ident;
    let fn_name_str = fn_ident.to_string();
    let fn_generics = &fn_sig.generics;
    let fn_inputs = &fn_sig.inputs;
    let fn_output = &fn_sig.output;

    let new_fn = quote! {
        #fn_vis fn #fn_ident #fn_generics(#fn_inputs) #fn_output {
            use std::time;

            let start = time::Instant::now();
            let fn_return_value = #fn_block;
            println!("fn {} costs {}", #fn_name_str, human_time::human_time(start.elapsed()));
            fn_return_value
        }
    };

    new_fn.into()
}
