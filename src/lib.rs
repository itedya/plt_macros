use syn::parse::Parse;
use proc_macro::TokenStream;
use std::str::FromStr;
use syn::{braced, parse_macro_input, token, Expr, FnArg, Ident, LitStr, Token};
use syn::__private::ToTokens;
use syn::punctuated::Punctuated;
use plt::prelude::*;

struct MacroInput {
    template_name: Ident,
    semicolon_1: Token![;],

    // This one cannot be a vec of expressions, because
    // the whitespaces will be omitted completely.
    template_content: LitStr,

    semicolon_2: Token![;],
    args: Punctuated<FnArg, Token![,]>,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // to be honest I don't know how half of this code works,
        // macros are a big black box for me. If you want
        // some explanation, docs for parse module in syn package
        // might be helpful https://docs.rs/syn/2.0.87/syn/parse/index.html

        Ok(Self {
            template_name: input.parse()?,
            semicolon_1: input.parse()?,
            template_content: input.parse()?,
            semicolon_2: input.parse()?,
            args: input.parse_terminated(FnArg::parse, Token![,])?,
        })
    }
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[proc_macro]
pub fn plt_template(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MacroInput);

    let template_name = input.template_name.to_string();
    let template_content = input.template_content.value();
    let args = input.args.iter().map(|arg| {
        arg.to_token_stream().to_string()
    }).collect::<Vec<String>>();

    let mut fsa = TextCodeFSA::new();
    let tokenized_template_content = fsa.run(template_content);

    let generated_function = generate_file(template_name, args, tokenized_template_content)
        .join(LINE_ENDING);
    let formatted_code = format_code(&generated_function);

    TokenStream::from_str(&formatted_code).unwrap()
}
