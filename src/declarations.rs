use crate::declare;
use macro_rules_attribute::apply;

#[apply(declare)]
mod __ {

    pub trait Fields {
        type Guards: Guards;
        #[apply(unfold)]
        type _VarField_;
    }

    const _: () = {
        use super::*;

        #[apply(generics)]
        impl<T: Guards> Fields for MGen!(T) {
            type Guards = T;
            #[apply(unfold)]
            type _VarField_ = T::_VarGuard_<_Type_>;
        }
    };

    pub trait Guards {
        #[apply(unfold)]
        type _VarGuard_<T: Tuckle>: Tuckle = !;
        #[apply(unfold)]
        type _VarUnit_: Tuckle = Self::_VarGuard_<()>;
    }

    #[apply(unfold)]
    #[allow(type_alias_bounds)]
    pub type _VarUnitOf_<T: Fields> = <T::Guards as Guards>::_VarUnit_;

    #[apply(unfold)]
    impl Guards for _Var_ {
        type _VarGuard_<T: Tuckle> = T;
    }

    impl Guards for Poly {
        #[apply(unfold)]
        type _VarGuard_<T: Tuckle> = T;
    }

    #[apply(unfold)]
    impl Guards for _PolyClass_ {
        #[apply(unfold)]
        type _MembGuard_<T: Tuckle> = T;
    }

    #[apply(unfold)]
    pub use _NameEnum_ as Enum;

    #[apply(unfold)]
    #[derive(__)]
    pub enum _NameEnum_<_VarUnit_, _VarField_> {
        _Var_ {
            __unit: _VarUnit_,
            _field_: _VarField_,
        },
    }

    #[apply(unfold)]
    pub use _NameWrap_ as Wrap;

    #[apply(unfold)]
    #[derive(__)]
    #[repr(transparent)]
    pub struct _NameWrap_<F: Fields>(pub Enum<_VarUnitOf_<F>, F::_VarField_>);
}
