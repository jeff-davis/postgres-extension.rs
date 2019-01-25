extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn pg_function_info_v1(item: TokenStream) -> TokenStream {
    let name = item.to_string();
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

    let code = format!("{v1_cc_code}\n{panic_handler_code}\n",
                       v1_cc_code = v1_cc_code,
                       panic_handler_code = panic_handler_code);

    eprintln!("code:\n{}", code);

    return code.parse().unwrap();
}
