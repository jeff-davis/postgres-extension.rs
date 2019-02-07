extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use syn::export::Span;
use proc_macro::{TokenStream,TokenTree};

/// Attribute macro does the following:
///
///  * creates a wrapper function that catches any rust panic, and
///    turns it into a postgres ERROR
///  * defines the wrapper function with the symbol name matching the
///    function
///
///
#[proc_macro_attribute]
pub fn pg_export(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    let mut opt_v1 = false;
    let mut nargs = 0;

    // should we use V1 calling convention or plain C?
    for opt in args {
        nargs += 1;
        if let TokenTree::Ident(ident) = opt {
            match ident.to_string().as_ref() {
                "V1" => { opt_v1 = true },
                "C" => { opt_v1 = false },
                s => panic!("invalid option: {}", s),
            }
        } else {
            panic!("option parse error");
        }
    }

    if nargs != 1 {
        panic!("must specify either 'C' or 'V1' as calling convention");
    }

    // parse into syn::ItemFn
    let synitem: syn::Item = parse_macro_input!(input);
    let itemfn: syn::ItemFn = match synitem {
        syn::Item::Fn(itemfn) => itemfn,
        _ => panic!("#[pg_function] must be used on a function"),
    };

    let name_ident: syn::Ident = itemfn.ident;
    let params = itemfn.decl.inputs;
    let return_type = itemfn.decl.output;

    if itemfn.constness != None {
        panic!("const functions cannot be exported");
    }
    if let syn::Visibility::Public(_) = itemfn.vis {
        panic!("function to export should not be marked 'pub'");
    }
    if itemfn.asyncness != None {
        panic!("async functions cannot be exported");
    }
    if itemfn.decl.variadic != None {
        panic!("variadic functions cannot be exported");
    }
    //TODO: reject generics
    //TODO: reject non repr(C) types in parameters or return
    //TODO: reject self params, captured only
    //TODO: copy params to arguments rather than hardcoding 'fcinfo'

    let name_str = &name_ident.to_string();
    let name_lit = syn::LitStr::new(name_str, Span::call_site());

    let v1_cc_str = &format!("pg_finfo_{}", name_str);
    let v1_cc_ident = syn::Ident::new(v1_cc_str, Span::call_site());
    let v1_cc_lit = syn::LitStr::new(v1_cc_str, Span::call_site());

    let panic_handler_str = &format!("rust_panic_handler_{}", name_str);
    let panic_handler_ident = syn::Ident::new(panic_handler_str, Span::call_site());

    if opt_v1 {
        let v1_cc_code = quote! {
            #[export_name=#v1_cc_lit]
            pub extern "C" fn #v1_cc_ident () ->
                &'static postgres_extension::fmgr::Pg_finfo_record
            {

                return &postgres_extension::fmgr::PG_FUNCTION_INFO_V1_DATA;
            }
        };

        output.extend(TokenStream::from(v1_cc_code));
    }

    let panic_handler_code = quote! {
        #[export_name=#name_lit]
        pub extern "C" fn #panic_handler_ident (#params) #return_type {
            rust_panic_handler!(#name_ident(fcinfo))
        }
    };

    output.extend(TokenStream::from(panic_handler_code));

    output
}
