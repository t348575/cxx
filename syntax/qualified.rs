use syn::ext::IdentExt;
use syn::parse::{Error, ParseStream, Result};
use syn::{Ident, LitStr, Token};

pub struct QualifiedName {
    pub segments: Vec<Ident>,
}

impl QualifiedName {
    pub fn parse_unquoted(input: ParseStream) -> Result<Self> {
        let allow_raw = true;
        parse_unquoted(input, allow_raw)
    }

    pub fn parse_quoted_or_unquoted(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            if lit.value().is_empty() {
                let segments = Vec::new();
                Ok(QualifiedName { segments })
            } else {
                lit.parse_with(|input: ParseStream| {
                    let allow_raw = false;
                    parse_unquoted(input, allow_raw)
                })
            }
        } else {
            Self::parse_unquoted(input)
        }
    }
}

fn parse_unquoted(input: ParseStream, allow_raw: bool) -> Result<QualifiedName> {
    let mut segments = Vec::new();
    let mut trailing_punct = true;
    let leading_colons: Option<Token![::]> = input.parse()?;
    while trailing_punct && input.peek(Ident::peek_any) {
        let mut ident = Ident::parse_any(input)?;
        if ident.to_string().starts_with("r#") {
            if !allow_raw {
                let msg = format!(
                    "raw identifier `{}` is not allowed in a quoted namespace",
                    ident,
                );
                return Err(Error::new(ident.span(), msg));
            }
            ident = ident.unraw();
        }
        segments.push(ident);
        let colons: Option<Token![::]> = input.parse()?;
        trailing_punct = colons.is_some();
    }
    if segments.is_empty() && leading_colons.is_none() {
        return Err(input.error("expected path"));
    } else if trailing_punct {
        return Err(input.error("expected path segment"));
    }
    Ok(QualifiedName { segments })
}
