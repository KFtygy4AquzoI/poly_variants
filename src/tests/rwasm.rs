use crate as poly_variants;
use crate::syntax::generic::generic;
use macro_rules_attribute::apply;

/*

macro_rules! skip {
    ($($t:tt)*) => {};
}

generic! {
    derive[],
    poly[
        ([Zero, Succ]: Nat)
    ],
    RWasm[ generics[], predicates[] ],
    block {
        Zero[
            fields[],
            out_0_auto[Yes], out_1_type[]
        ],
        Succ[
            fields[
                (
                    fid0, _0_wild[yes], _1_name[],
                    ftype[
                        _0_path[],
                        _1_macro[
                            [Repr]
                            [
                                _0_path[], _1_macro[],
                                _2_impl[Nat],
                                _3_dyn[]
                            ]
                        ],
                        _2_impl[], _3_dyn[]
                    ]
                )
            ],
            out_0_auto[Yes], out_1_type[]
        ]
    }
}

use crate::poly_variants::draft01::make_makers;

#[macro_export]
macro_rules! with_path { ($x:ident, $($p:tt)+) => {
    $($p)*! { $x, $crate::poly_variants::tests::rwasm }
}}
pub use with_path;

with_path! { solve_path, make_makers }
wname! { solve_name, make_makers_with_path }
unr_v! { solve_var, make_makers_with_name }

pub struct KUKU;

const _: () = {
  let _ = crate::poly_variants::tests::rwasm::RWasm::Wrap(());
  let _ = ZeroPat! { mytest };
};

#[apply(skip)]
pub trait Lang: Sized + Fields {
    type Wrap<F: Fields>: RWasm<Self> = !;
    type Inner<P: Guards>: Types = Leaf;
    type Out: Guards = Poly;
}

#[apply(skip)]
macro_rules! Repr { ($($t:tt)*) => { Lng::Wrap<impl Lang<Out = $($t)*>> } }

#[apply(skip)]
#[super_trait(Lang: Fields)]
trait RWasm<Lng: Lang> {
    fn zero() -> Repr!(impl Nat);
    fn succ(_: &Repr!(impl Nat)) -> Repr!(impl Nat);

    fn local_depth(_: &Repr!(impl Nat));

    fn i64_const<S: Stack>(_: Repr!(S), _: i64) -> Repr!(impl Alt<impl Push<S, i64>, impl Error>>);

    fn local_get<S: Stack, L: LocalDepth>(
        _: Repr!(S),
        _: &Repr!(L),
    ) -> Repr!(impl Alt<impl Push<S, impl Value>, impl Error>>);
}

*/
