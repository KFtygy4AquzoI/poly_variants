use crate::poly_variants;
use macro_rules_attribute::apply;

use Expr::*;

#[allow(type_alias_bounds)]
pub type WrapOf<Ty: Types, P: Guards> = Ty::Wrap<Gen<P, Ty::Inner<P>>>;

pub trait Types: Sized {
    type Out: Guards = Poly;
    type Inner<P: Guards>: Types = Leaf;
    type Wrap<F: Fields> = !;
}

pub struct Leaf;

impl Types for Leaf {}

#[apply(poly_variants)]
#[poly([Add]: Bytecode)]
#[poly([Add, Jump]: Instr)]
#[poly([Push, Builder, Composer, AddInstr, PutLabel]: Fragment)]
#[poly([Composer]: Composition)]
#[poly([Builder]: Build)]
#[poly([PutLabel]: Location)]
#[poly([PutLabel]: Address)]
pub enum Expr<Ty: super::Types> {
    Push {
        a: u32,
    },
    Add,

    #[rule(Address -> Bytecode)]
    Jump {
        a: WrapOf<Ty, PolyLocation>,
    },

    Builder,

    #[rule(Build -> Bytecode -> Build)]
    AddInstr {
        a: WrapOf<Ty, PolyFragment>,
        b: WrapOf<Ty, PolyInstr>,
    },
    #[rule(Build -> Address)]
    PutLabel {
        a: WrapOf<Ty, PolyFragment>,
    },

    Composer,
    #[rule(Build -> -> Build)]
    Concat {
        a: WrapOf<Ty, PolyFragment>,
        b: WrapOf<Ty, PolyComposition>,
    },
}
