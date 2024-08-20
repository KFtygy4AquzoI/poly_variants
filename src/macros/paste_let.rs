use macro_rules_attribute::apply;

/*

#[macro_rules_rec::recursive]
macro_rules! paste_let {

    (
        @prepared [$( $bind:ident [$solved:tt] ),*]
        macro_rules! $name:ident { () => { $($t:tt)* } $(;)? }
    ) => {
        macro_rules! $name {
            (@solved $([$$$bind:ident])* ) => { $($t)* };
            () => { $name! { @solved $([$solved])* } };
        }
    };
    (
        $(#[paste(let $bind:ident = $($pat:tt)*)])* macro_rules $($t:tt)+
    ) => { paste::paste! {
        $self! { @prepared [$( $bind [[<$($pat)*>]] ),*] macro_rules $($t)* }
    }};
}

*/

/*
#[macro_rules_rec::recursive]
macro_rules! paste_let {
    (
        #[paste_letter(let _A_ =
}

pub mod tests {
    use super::*;

    pub struct QQQQ;

    #[apply(paste_let)]
    #[paste_letter(let _A_ = alpha)]
    macro_rules! __ {
        () => {
            mod $_A_ {}
        };
    }
*/

/*
    #[apply(paste_let)]
    #[paste(let testabc = mytest _ _ test)]
    #[paste(let __ = testabc _ mylet)]
    macro_rules! _Name_ {
        () => {
            mod $testabc {}
        };
    }
*/

//mylet!();
//}
