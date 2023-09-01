use derive_syn_parse::Parse;
use syn::{Attribute, Block, Signature, Token, Type, Visibility};

#[derive(Debug, Parse)]
pub struct YieldFn {
    #[call(Attribute::parse_outer)]
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub yield_token: Token![yield],
    pub yield_ty: Type,
    pub block: Block,
}

pub struct YieldClosure;
