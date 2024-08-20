use crate::poly_variants;
use macro_rules_attribute::apply;

use Expr::*;

#[allow(type_alias_bounds)]
pub type WrapOf<Ty: Types, P: Guards> = Ty::Wrap<Gen<P, Ty::Inner<P>>>;

pub trait Types: Sized {
    type Inner<P: Guards>: Types = Leaf;
    type Wrap<F: Fields> = !;
    type Lambda: FnOnce(WrapOf<Self, PolyValue>) -> WrapOf<Self, PolyValue> =
        Box<dyn FnOnce(WrapOf<Self, PolyValue>) -> WrapOf<Self, PolyValue>>;
}

pub struct Leaf;

impl Types for Leaf {}

#[apply(poly_variants)]
#[poly([U32]: Leaf)]
#[poly([U32, Add, Apply]: Value)]
pub enum Expr<Ty: super::Types> {
    U32 {
        a: u32,
    },
    Add {
        a: WrapOf<Ty, PolyValue>,
        b: WrapOf<Ty, PolyValue>,
    },
    Lambda {
        a: Ty::Lambda,
    },
    Apply {
        a: WrapOf<Ty, Lambda>,
        b: WrapOf<Ty, PolyValue>,
    },
}
