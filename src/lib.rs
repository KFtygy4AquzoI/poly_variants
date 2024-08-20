#![feature(proc_macro_hygiene)]
#![feature(macro_metavar_expr)]
#![feature(stmt_expr_attributes)]
#![feature(adt_const_params)]
#![feature(generic_const_items)]
#![feature(associated_type_defaults)]
#![feature(never_type)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(exhaustive_patterns)]
#![feature(inherent_associated_types)]
//#![feature(generic_const_exprs)]
#![feature(log_syntax)]

use macro_rules_attribute::apply;

mod declarations;
mod flat_declarations;
pub mod syntax;

//#[cfg(test)]
pub mod tests;

pub mod macros;

//use crate::declare;

#[macro_export]
macro_rules! poly_variants {(
/* **** **** **** **** PATTERN INSIDE MACRO RULES **** **** **** **** */

$(#[derive($De:ident$(, $Dn:ident)*)])?
$(#[poly([$($Memb:ident),+]: $Class:ident)])*
pub enum $Name:ident$(<$($G:ident$(:
                                           $P0:ident$(::$Q0:ident)*$(<$($Tg0:ty),+>)?
                                       $(+ $P1:ident$(::$Q1:ident)*$(<$($Tg1:ty),+>)? )*
                                  )?
                        ),+ $(,)?
                      >)?
    $(where
         $($Wt:ty:     $P2:ident$(::$Q2:ident)*$(<$($Tg2:ty),+>)?
                   $(+ $P3:ident$(::$Q3:ident)*$(<$($Tg3:ty),+>)? )*
         ),* $(,)?
     )?
{
 $(

    $(#[$($vattr:tt)*])*
    $Var:ident $({ $( $field:ident: $Type:ty ),+ $(,)? })?

  ),* $(,)?
}

/* **** **** **** **** PATTERN INSIDE MACRO RULES **** **** **** **** */
) => { paste::paste! {
/* **** **** **** **** BODY INSIDE MACRO RULES    **** **** **** **** */

#[allow(non_snake_case)]
pub mod $Name {
/* **** **** **** **** MODULE INSIDE MACRO RULES  **** **** **** **** */

use macro_rules_attribute::apply;

use count_macro::count;
use std::marker::PhantomData as Ph;
use std::fmt::Debug;
use crate::{skip, pass};

use paste::paste;

use $crate::{declarations::*, flat_declarations::*};

macro_rules! generics { ($$($$code:tt)*) => { $crate::apply_generics! { [$$($$code)*]
   [$($($G$(:$P0$(::$Q0)*$(<$($Tg0),*>)?$( + $P1$(::$Q1)*$(<$($Tg1),*>)?)*)?),*)?]
   [$($($Wt:$P2$(::$Q2)*$(<$($Tg2),*>)?$( + $P3$(::$Q3)*$(<$($Tg3),*>)?)*),*)?]
}}}

macro_rules! MGen { ($__Ty:ty) => { Gen<$__Ty $($(, $G)*)?> } }

$crate::macro_rules_stack! { unfold {
         (      pub use   _NameWrap_   as Wrap;
    ) => {      pub use [<$Name Wrap>] as Wrap;
    };
         (      pub use   _NameEnum_   as Enum;
    ) => {      pub use [<$Name Enum>] as Enum;
    };
         (          #[derive(    __     )]
    ) => {$(        #[derive($De$(,$Dn)*)]        )?
    };
         (      #[allow(type_alias_bounds)]
                pub type   _VarUnitOf_  <T: Fields> = <T::Guards as Guards>::  _VarUnit_;
    ) => {$(
                #[allow(type_alias_bounds)]
                pub type [<$Var UnitOf>]<T: Fields> = <T::Guards as Guards>::[<$Var Unit>]; )*
    };
         (          type   _VarGuard_  <T: Tuckle>: Tuckle = !;
    ) => {$(        type [<$Var Guard>]<T: Tuckle>: Tuckle = !;               )*
    };
         (          type   _VarGuard_  <T: Tuckle> = T;
    ) => {$(        type [<$Var Guard>]<T: Tuckle> = T;                       )*
    };
         (          type   _VarUnit_:   Tuckle = Self::  _VarGuard_  <()>;
    ) => {$(        type [<$Var Unit>]: Tuckle = Self::[<$Var Guard>]<()>;    )*
    };
         (          type   _VarField_;
    ) => {$($($(    type [<$Var Field $field:camel>];                                       )*)?)*
    };
         (          type   _VarField_                = $T:ident ::  _VarGuard_  <_Type_>;
    ) => {$($($(    type [<$Var Field $field:camel>] = $T       ::[<$Var Guard>]<$Type> ;   )*)?)*
    };
         (      pub struct   _NameWrap_  <F: Fields>(
                        pub Enum<    _VarUnitOf_  <F>,     F::  _VarField_                            >);
    ) => {      pub struct [<$Name Wrap>]<F:       Fields>(
                        pub Enum<$([<$Var UnitOf>]<F>, $($(F::[<$Var Field $field:camel>],)*)?)*      >);
    };
         (      pub enum   _NameEnum_  <     _VarUnit_,         _VarField_ $$(,)?                     >
    ) => {      pub enum [<$Name Enum>]< $([<$Var Unit>], $($([<$Var Field $field:camel>],)*)?)*      >
    };
         (          impl Guards for  _PolyClass_ {      #[apply(unfold)]
                                                        type   _MembGuard_  <T: Tuckle> = T;      }
    ) => {$(        impl Guards for [<Poly $Class>] { $(type [<$Memb Guard>]<T: Tuckle> = T;)*    }   )*
    };
         (          impl Guards for _Var_ { type  _VarGuard_  <T: Tuckle> = T; }
    ) => {$(        impl Guards for $Var { type [<$Var Guard>]<T: Tuckle> = T; })*
    };
        ({          _Var_ { __unit:   _VarUnit_,        _field_:  _VarField_   $$(,)?             },  }
    ) => {$(        $Var  { __unit: [<$Var Unit>], $($( $field: [<$Var Field $field:camel>] ),*)* },  )*
    };
}}

//pub trait PolyTrait {
//}

impl Poly {
    $(pub type $Class = [<Poly $Class>];)*
}

#[apply(generics)]
pub struct Gen<T: Guards>(    pub Ph<(    T,    $($($G),*)?    )>    );

pub trait Tuckle$(: $De $( + $Dn )*)? {}
impl<A$(: $De $( + $Dn )*)?> Tuckle for A {}

const _: () = {
  use super::*;

/*
$(#[apply(generics)]
impl FnOnce<($($($Type,)*)?)> for MGen!($Var) {
       type Output = Wrap<MGen!($Var)>;
       #[inline(always)]
       extern "rust-call" fn call_once(self, args: ($($($Type,)*)?)) -> Self::Output { count! {
          Wrap(Enum::$Var { __unit: () $($(, $field: args._int_)*)? })
       }}
})*
*/

};

$(macro_rules! [<$Var Pat>] { ($$($$t:tt)*) => { $Name::Wrap($Name::Enum::$Var { $$($$t)* }) } }
pub(crate) use [<$Var Pat>];)*

$(macro_rules! [<$Var Cons>] { ($$($$t:tt)*) => { $Name::Wrap($Name::Enum::$Var { __unit: (), $$($$t)* }) } }
pub(crate) use [<$Var Cons>];)*

$(macro_rules! [<$Var Bind>] { ( [$$($$u:tt)*], $($($$$field:ident),*)? $$($$t:tt)* ) => {
    Wrap(Enum::$Var { $($($$$field, )*)? __unit: $$($$u)* })
}}
pub(crate) use [<$Var Bind>];)*

$(macro_rules! [<$Class:snake _upcast_into_poly>] { ($value:ident) => {
    match $value {
        $([<$Memb Bind>]!([_], a, b, c, d, e, f, g) => [<$Memb Bind>]!([()], a, b, c, d, e, f, g),)*
    }
}})*

$(macro_rules! [<$Class:snake _downcast_from_poly>] { ($value:ident) => {
    match $value {
        $([<$Memb Bind>]!([_], a, b, c, d, e, f, g) => Ok([<$Memb Bind>]!([()], a, b, c, d, e, f, g)),)*
        _ => Err(Mismatched)
    }
}})*

$(macro_rules! [<$Var:snake _rebind>] { ($value:ident) => {
    let Wrap(Enum::$Var { $($($field, )*)? __unit: () }) = $value;
    Wrap(Enum::$Var { $($($field, )*)? __unit: () })
}}
pub(crate) use [<$Var:snake _rebind>];)*

$(macro_rules! [<$Var:snake _try_rebind>] { ($value:ident) => {
    if let Wrap(Enum::$Var { $($($field, )*)? __unit: () }) = $value {
        Ok(Wrap(Enum::$Var { $($($field, )*)? __unit: () }))
    } else {
        Err(Mismatched)
    }
}}
pub(crate) use [<$Var:snake _try_rebind>];)*

flat_declarations!(
    ( MGen ),
    ( pass, skip, skip, skip ),
    ( $Name, __, __, __ ), ( __ ), ( __, __, __, __, __, __ )
);

$(flat_declarations!(
    ( MGen ),
    ( skip, pass, skip, skip ),
    ( $Name, $Var, __, __ ),
    ( __ ),
    (
        [<$Var:snake _rebind>],
        [<$Var:snake _try_rebind>],
        __, __, __, __
    )
);)*

$(flat_declarations!(
    ( MGen ),
    ( skip, skip, pass, skip ),
    ( $Name, __, $Class, __ ),
    ( [<Poly $Class>] ),
    (
        __, __,
        [<$Class:snake _upcast_into_poly>],
        [<$Class:snake _downcast_from_poly>],
        __, __
    )
);)*

$($(flat_declarations!(
    ( MGen ),
    ( skip, skip, skip, pass ),
    ( $Name, __, $Class, $Memb ),
    ( [<Poly $Class>] ),
    (
        __, __, __, __,
        [<$Memb:snake _rebind>],
        [<$Memb:snake _try_rebind>]
    )
);)*)*

/* **** **** **** **** MODULE INSIDE MACRO RULES  **** **** **** **** */
}
/* **** **** **** **** BODY INSIDE MACRO RULES **** **** **** **** */
}}}
