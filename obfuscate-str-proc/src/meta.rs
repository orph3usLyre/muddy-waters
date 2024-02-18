use syn::parse::{Parse, ParseStream, Result};
use syn::{Expr, ExprLit, Ident, Visibility};
use syn::{ItemStatic, LitStr};

pub struct NonObfuscatedTexts {
    pub texts: Vec<NonObfuscatedText>,
}

pub struct NonObfuscatedText {
    pub visibility: Visibility,
    pub variable_name: Ident,
    pub text: LitStr,
}

impl Parse for NonObfuscatedTexts {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut texts = Vec::new();
        while let Ok(non_obfuscated_text) = input.parse() {
            texts.push(non_obfuscated_text);
        }
        Ok(NonObfuscatedTexts { texts })
    }
}

impl Parse for NonObfuscatedText {
    fn parse(input: ParseStream) -> Result<Self> {
        let ItemStatic {
            vis: visibility,
            mutability,
            ident: variable_name,
            expr,
            ..
        } = input.parse()?;

        if let syn::StaticMutability::Mut(_) = mutability {
            panic!("Expression cannot be mutable");
        }
        let Expr::Lit(ExprLit {
            lit: syn::Lit::Str(text),
            ..
        }) = *expr
        else {
            panic!("Expression must be of type &'static str");
        };

        Ok(NonObfuscatedText {
            visibility,
            variable_name,
            text,
        })
    }
}
