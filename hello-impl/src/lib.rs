use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
pub fn seq_failed(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as failed::Seq);
    // It seems the TokenStream yielded from cursor in parse fn cannot be passed
    input.tokens.into()
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as succeeded::Seq);
    input.tokens().into()
}

mod failed {
    use super::{tokens, TokenStream2};

    #[allow(dead_code)]
    pub struct Seq {
        ident:       syn::Ident,
        brace_token: syn::token::Brace,
        pub tokens:  TokenStream2,
    }

    impl syn::parse::Parse for Seq {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let ident = input.parse()?;
            let content;
            let brace_token = syn::braced!(content in input);

            // *** Here is the difference ***
            let cursor = content.cursor();
            let tokens = tokens(cursor);
            dbg!(&tokens);
            // ******************************

            Ok(Seq { ident,
                     brace_token,
                     tokens })
        }
    }
}

mod succeeded {
    use super::{tokens, TokenStream2};

    #[allow(dead_code)]
    pub struct Seq {
        ident:       syn::Ident,
        brace_token: syn::token::Brace,
        tokens:      TokenStream2,
    }

    impl syn::parse::Parse for Seq {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let ident = input.parse()?;
            let content;
            let brace_token = syn::braced!(content in input);

            // *** Here is the difference ***
            let tokens = content.parse()?;
            // ******************************

            Ok(Seq { ident,
                     brace_token,
                     tokens })
        }
    }

    impl Seq {
        pub fn tokens(self) -> TokenStream2 {
            let buf = syn::buffer::TokenBuffer::new2(self.tokens);
            tokens(buf.begin())
        }
    }
}

fn tokens(cursor: syn::buffer::Cursor) -> TokenStream2 { cursor.token_stream() }
