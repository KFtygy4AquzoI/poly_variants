mod apply_generics;
mod macro_rules_stack;
mod make_macro_kit_up;
mod paste_let;

#[macro_export]
macro_rules! declare { (mod __ { $($code:tt)* }) => {
    macro_rules! declarations { () => { $($code)* } }
    pub(crate) use declarations;
}}

#[macro_export]
macro_rules! pass { ($($t:tt)*) => {$($t)*} }

#[macro_export]
macro_rules! skip {
    ($($t:tt)*) => {};
}
