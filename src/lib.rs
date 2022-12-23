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
            {
                fn human_time(d: Duration) -> String {
                    human_time_with_format(
                        d,
                        |n, unit| format!("{}{}",n , unit),
                        |acc, item| format!("{},{}", acc, item),
                    )
                }

                fn human_time_with_format<F, F1>(d: Duration, time_fmt: F, res_fmt: F1) -> String
                where
                    F: Fn(u128, &str) -> String,
                    F1: Fn(String, String) -> String,
                {
                    let mut map: Vec<(u128, &str)> = Vec::new();
                    let mut ms = d.as_millis();
                    for (unit, n_ms) in [
                        ("d", 86400000),
                        ("h", 3600000),
                        ("m", 60000),
                        ("s", 1000),
                        ("ms", 1),
                    ] {
                        map.push((ms / n_ms, unit));
                        ms %= n_ms;
                    }

                    match map
                        .into_iter()
                        .filter_map(|(n, u)| if n > 0 { Some(time_fmt(n, u)) } else { None })
                        .reduce(res_fmt)
                    {
                        Some(val) => val,
                        None => time_fmt(0, "ms"),
                    }
                }

                println!("fn {} costs {}", #fn_name_str, human_time(start.elapsed()));

            }
            fn_return_value
        }
    };

    new_fn.into()
}
