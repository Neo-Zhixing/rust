/// `tt` crate defines a `TokenTree` datastructure: this is the interface (both
/// input and output) of macros. It closely mirrors `proc_macro` crate's
/// `TokenTree`.

macro_rules! impl_froms {
    ($e:ident: $($v:ident), *) => {
        $(
            impl From<$v> for $e {
                fn from(it: $v) -> $e {
                    $e::$v(it)
                }
            }
        )*
    }
}

use std::fmt;

use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenId(pub u32);

impl TokenId {
    pub const fn unspecified() -> TokenId {
        TokenId(!0)
    }
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Leaf(Leaf),
    Subtree(Subtree),
}
impl_froms!(TokenTree: Leaf, Subtree);

#[derive(Debug, Clone)]
pub enum Leaf {
    Literal(Literal),
    Punct(Punct),
    Ident(Ident),
}
impl_froms!(Leaf: Literal, Punct, Ident);

#[derive(Debug, Clone)]
pub struct Subtree {
    pub delimiter: Delimiter,
    pub token_trees: Vec<TokenTree>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delimiter {
    Parenthesis,
    Brace,
    Bracket,
    None,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub text: SmolStr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Punct {
    pub char: char,
    pub spacing: Spacing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    Alone,
    Joint,
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub text: SmolStr,
    pub id: TokenId,
}

impl fmt::Display for TokenTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenTree::Leaf(it) => fmt::Display::fmt(it, f),
            TokenTree::Subtree(it) => fmt::Display::fmt(it, f),
        }
    }
}

impl fmt::Display for Subtree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (l, r) = match self.delimiter {
            Delimiter::Parenthesis => ("(", ")"),
            Delimiter::Brace => ("{", "}"),
            Delimiter::Bracket => ("[", "]"),
            Delimiter::None => ("", ""),
        };
        f.write_str(l)?;
        let mut needs_space = false;
        for tt in self.token_trees.iter() {
            if needs_space {
                f.write_str(" ")?;
            }
            needs_space = true;
            match tt {
                TokenTree::Leaf(Leaf::Punct(p)) => {
                    needs_space = p.spacing == Spacing::Alone;
                    fmt::Display::fmt(p, f)?
                }
                tt => fmt::Display::fmt(tt, f)?,
            }
        }
        f.write_str(r)?;
        Ok(())
    }
}

impl fmt::Display for Leaf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leaf::Ident(it) => fmt::Display::fmt(it, f),
            Leaf::Literal(it) => fmt::Display::fmt(it, f),
            Leaf::Punct(it) => fmt::Display::fmt(it, f),
        }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.text, f)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.text, f)
    }
}

impl fmt::Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.char, f)
    }
}
