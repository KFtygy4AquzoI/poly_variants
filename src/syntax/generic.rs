#[macro_export]
macro_rules! poly_variants_generic { (

  // derive[]
  // derive[Debug]
  // derive[Clone, Copy]
  derive[$(    $Dre:ident $(, $Drn:ident )*    )?],

  // poly[]
  // poly[([U8, U32]: Value)]
  // poly[([Add]: Binop), ([Lambda]: Function)]
  poly[$((    [$($Memb:ident),+]: $Class:ident    )),*],

  // MyExpr[ generics[], predicated[] ]
  $Name:ident[

      // generics[ (A), (B) ]
      // generics[ (C [Clone] [Copy]) ]
      generics[$($((
          $Gid:ident $(    [$($Gtre:tt)*] $( [$($Gtrn:tt)*] )+    )?
      )),+)?],

      // predicates[ ([A]: [Copy]) ]
      // predicates[ ([T::Field]: [Clone]), ([A]: [Copy] [std::fmt::Debug]) ]
      predicates[$($((
          [$($Pty:tt)*]: [$($Ptre:tt)*] $( [$($Ptrn:tt)*] )*
      ))+)?]

  ],

  // block { Zero[ fields[], out_0_auto[Yes], out_1_type[] ] }
  block {$(

      $Var:ident[
          // fields[ (fid0, _0_wild[yes], _1_name[], ftype[u32]) ]
          // fields[ (fid0, _0_wild[], _1_name[a], ftype[impl Nat]), (fid1, _0_wild[yes] ...
          fields[$($((
              $fid:ident,
              _0_wild[$($fwild:ident)?],
              _1_name[$($fname:ident)?],
              ftype[$($Ft:tt)*]
          )),+)?],
          out_0_auto[$($Oau:ident)?],
          out_1_type[$($Oty:tt)*]
      ]

  ),+}

) => {
/* **** NESTED **** */

pub mod $Name {
  pub struct Wrap(pub ());
  impl Wrap {
    pub fn new(a: ()) -> Self {
      Wrap(())
    }
  }
  //macro_rules! Wrap { ($$($$t:tt)*) => { Wrap2($$($$t)*) } }
  //pub(crate) use Wrap;
}

macro_rules! unr_v       { ($x:ident, $$($$p:tt)+) => { $(        $$($$p)*! { $x,       $Var                 }    )*     }}
macro_rules! unr_v_wfi   { ($x:ident, $$($$p:tt)+) => { $(        $$($$p)*! { $x,       $Var, $($($fid),*)?  }    )*     }}
macro_rules! unr_c       { ($x:ident, $$($$p:tt)+) => { $(        $$($$p)*! { $x,     $Class                 }    )*     }}
macro_rules! unr_m       { ($x:ident, $$($$p:tt)+) => { $($(      $$($$p)*! { $x,      $Memb                 }    )*)*   }}
macro_rules! unr_v_fi    { ($x:ident, $$($$p:tt)+) => { $($($(    $$($$p)*! { $x,       $Var, $fid           }    )*)?)* }}
macro_rules! unr_v_fi_ft { ($x:ident, $$($$p:tt)+) => { $($($(    $$($$p)*! { $x,       $Var, $fid, $($Ft)*  }    )*)?)* }}
macro_rules! wname       { ($x:ident, $$($$p:tt)+) => {           $$($$p)*! { $x,      $Name                 }           }}
macro_rules! wgid        { ($x:ident, $$($$p:tt)+) => {           $$($$p)*! { $x, $($(  $Gid            ),*)?}           }}
macro_rules! wderive     { ($x:ident, $$($$p:tt)+) => {           $$($$p)*! { $x, $(    $Dre $(, $Drn  )*  )?}           }}

macro_rules! equip_generics { ($$($$code:tt)*) => { $crate::apply_generics! { [$$($$code)*]
   [$($(   $Gid$(: $($Gtre)* $( + $($Gtrn)* )* )? ),*)?]
   [$($( $($Pty)*: $($Ptre)* $( + $($Ptrn)* )*    ),*)?]
}}}

/* **** NESTED **** */
}}
pub use poly_variants_generic as generic;
