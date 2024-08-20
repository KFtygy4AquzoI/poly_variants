use crate::poly_variants;
use macro_rules_attribute::apply;

#[apply(poly_variants)]
#[derive(Debug, Clone)]
#[poly([Var1, Var3]: Large)]
#[poly([Var3, Var4]: Huge)]
pub enum Simple<A: std::convert::From<u32> + std::convert::From<A>, B, C>
where
    B: Clone,
    C: From<A> + std::clone::Clone,
    A: std::fmt::Debug + Clone,
    B: std::fmt::Debug + Clone,
    C: std::fmt::Debug + Clone,
{
    Var0 { a: u8, b: u16 },
    Var1 { a: u32 },
    Var2,
    Var3 { a: u64, b: u64, c: u64, d: u64 },
    Var4 { a: u128 },
    Var5 { a: A, b: B, c: C },
}

macro_rules! layout_test {
    ($v:expr, $size:expr, $align:expr) => {
        let layout = Layout::for_value($v);
        assert_eq!(layout.size(), $size);
        assert_eq!(layout.align(), $align);
    };
}

#[test]
fn test_variants_01() {
    use std::alloc::Layout;
    use Simple::*;
    //let var0 = Simple::Gen::<Var0, u32, u32, u32>(std::marker::PhantomData)(1_u8, 2_u16);
    let var0: Wrap<Gen<Var0, u32, u32, u32>> = Simple::Var0Pat! { a: 1_u8, b: 2_u16, __unit: () };
    let Wrap(Enum::Var0 { a, .. }) = var0;
    let Simple::Var0Pat! { b, .. } = var0;
    println!("{:#?}", a);
    println!("{:#?}", b);

    layout_test!(&var0, 4, 2);

    let tagged: Wrap<Gen<Poly, u32, u32, u32>> = Simple::Var0Pat! { a: 3, b: 4, __unit: () };
    let tagless: Wrap<Gen<Var0, u32, u32, u32>> = Simple::Var0Pat! { a: 3, b: 4, __unit: () };
    let retagged: Wrap<Gen<Poly, u32, u32, u32>> = tagless.into();

    layout_test!(&tagged, 48, 16);

    let res_untagged: Result<Wrap<Gen<Var0, u32, u32, u32>>, _> = retagged.clone().try_into();
    assert!(res_untagged.is_ok());

    let res_untagged: Result<Wrap<Gen<Var1, u32, u32, u32>>, _> = retagged.try_into();
    assert!(res_untagged.is_err());

    layout_test!(&res_untagged, 8, 4);

    let var1: Wrap<Gen<Var1, u32, u32, u32>> = Simple::Var1Cons! { a: 123 };

    layout_test!(&var1, 4, 4);

    let large: Wrap<Gen<Poly::Large, u32, u32, u32>> = var1.into();
    layout_test!(&large, 40, 8);

    let poly: Wrap<Gen<Poly, u32, u32, u32>> = large.into();

    let res_dc: Result<Wrap<Gen<Poly::Large, u32, u32, u32>>, _> = poly.clone().try_into();
    assert!(res_dc.is_ok());

    let res_dc: Result<Wrap<Gen<Poly::Huge, u32, u32, u32>>, _> = poly.try_into();
    assert!(res_dc.is_err());

    layout_test!(&res_dc, 48, 16);

    let var3: Wrap<Gen<Var3, u32, u32, u32>> = Simple::Var3Cons! { a: 123, b: 0, c: 0, d: 0 };
    let huge: Wrap<Gen<Poly::Huge, u32, u32, u32>> = var3.into();
    layout_test!(&huge, 48, 16);
    let poly: Wrap<Gen<Poly, u32, u32, u32>> = huge.into();

    let res_dc: Result<Wrap<Gen<Poly::Large, u32, u32, u32>>, _> = poly.clone().try_into();
    assert!(res_dc.is_ok());

    let res_dc: Result<Wrap<Gen<Poly::Huge, u32, u32, u32>>, _> = poly.try_into();
    assert!(res_dc.is_ok());
}
