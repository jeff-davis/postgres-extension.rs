extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

fn get_fn_name(input: TokenStream) -> String {
    return syn::parse::<syn::ItemFn>(input).unwrap().ident.to_string();
}

#[proc_macro_attribute]
pub fn postgres_function(_args: TokenStream, input: TokenStream) -> TokenStream {
    let name = get_fn_name(input.clone());
    let v1_cc_name = format!("pg_finfo_{}", name);
    let panic_handler_name = format!("rs_panic_handler_{}", name);

    let v1_cc_code = format!(r###"
        #[export_name="{v1_cc_name}"]
        pub extern "C" fn {v1_cc_name}() ->
            &'static postgres_extension::fmgr::Pg_finfo_record {{

            return &postgres_extension::fmgr::PG_FUNCTION_INFO_V1_DATA;
        }}
        "###, v1_cc_name = v1_cc_name);

    let panic_handler_code = format!(r###"
        #[export_name="{name}"]
        pub extern "C" fn {panic_handler_name}(
            fcinfo : postgres_extension::fmgr::FunctionCallInfo) -> Datum {{
            rust_panic_handler!({name}(fcinfo))
        }}
        "###, name=name, panic_handler_name=panic_handler_name);

    let code = format!("{v1_cc_code}\n{panic_handler_code}\n{input}\n",
                       v1_cc_code = v1_cc_code,
                       panic_handler_code = panic_handler_code,
                       input = input.to_string());

    eprintln!("code:\n{}", code);

    return code.parse().unwrap();
}
