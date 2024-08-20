// Flat declarations can be formatted using rustfmt
macro_rules! flat_declarations {
    (
        ( $Gen:ident ),
        ( $is_mod:ident, $is_var:ident, $is_class:ident, $is_memb_class:ident ),
        ( $Name:ident, $Var:ident, $Class:ident, $Memb:ident ),
        ( $_PolyClass_:ident ),
        (
            $_var_rebind_:ident,             $_var_try_rebind_:ident,
            $_class_upcast_into_poly_:ident, $_class_downcast_from_poly_:ident,
            $_memb_rebind_:ident,            $_memb_try_rebind_:ident
        )
    ) => {
        #[apply($is_mod)]
        pub mod error {
            #[derive(Debug, Clone, Copy)]
            pub struct Mismatched;
        }
        #[apply($is_mod)]
        use error::*;

        #[apply($is_var)]
        #[derive(Debug, Clone, Copy)]
        pub struct $Var;

        #[apply($is_mod)]
        #[derive(Debug, Clone, Copy)]
        pub struct Poly;

        #[apply($is_class)]
        #[derive(Debug, Clone, Copy)]
        pub struct $_PolyClass_;

        #[apply($is_class)]
        pub type $Class = $_PolyClass_;

        #[apply($is_mod)]
        declarations!();

        #[apply($is_var)]
        #[apply(generics)]
        impl From<Wrap<$Gen!($Var)>> for Wrap<$Gen!(Poly)> {
            fn from(value: Wrap<$Gen!($Var)>) -> Self {
                $_var_rebind_! { value }
            }
        }

        #[apply($is_class)]
        #[apply(generics)]
        impl From<Wrap<$Gen!($_PolyClass_)>> for Wrap<$Gen!(Poly)> {
            fn from(value: Wrap<$Gen!($_PolyClass_)>) -> Self {
                $_class_upcast_into_poly_! { value }
            }
        }

        #[apply($is_memb_class)]
        #[apply(generics)]
        impl From<Wrap<$Gen!($Memb)>> for Wrap<$Gen!($_PolyClass_)> {
            fn from(value: Wrap<$Gen!($Memb)>) -> Self {
                $_memb_rebind_! { value }
            }
        }

        #[apply($is_var)]
        #[apply(generics)]
        impl TryFrom<Wrap<$Gen!(Poly)>> for Wrap<$Gen!($Var)> {
            type Error = Mismatched;
            fn try_from(value: Wrap<$Gen!(Poly)>) -> Result<Self, Self::Error> {
                $_var_try_rebind_! { value }
            }
        }

        #[apply($is_class)]
        #[apply(generics)]
        impl TryFrom<Wrap<$Gen!(Poly)>> for Wrap<$Gen!($_PolyClass_)> {
            type Error = Mismatched;
            fn try_from(value: Wrap<$Gen!(Poly)>) -> Result<Self, Self::Error> {
                $_class_downcast_from_poly_! { value }
            }
        }

        #[apply($is_memb_class)]
        #[apply(generics)]
        impl TryFrom<Wrap<$Gen!($_PolyClass_)>> for Wrap<$Gen!($Memb)> {
            type Error = Mismatched;
            fn try_from(value: Wrap<$Gen!($_PolyClass_)>) -> Result<Self, Self::Error> {
                $_memb_try_rebind_! { value }
            }
        }
    };
}
pub(crate) use flat_declarations;
