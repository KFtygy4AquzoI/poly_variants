use macro_rules_attribute::apply;

#[macro_export]
#[cps::cps]
#[macro_rules_rec::recursive]
macro_rules! make_macro_kit_up {
    ($get_module_path:ident) =>
    let $($module_path:tt)* = $get_module_path!() in
    {
        $self! { @parse $($module_path)* }
    };
    (@parse $pe:ident$(::$pn:ident)*) => { paste::paste! {
        $self! { @prepare [$pe$(::$pn)*] [<__ $($pn __)*>] }
    }};
    (@prepare [$($module_path:tt)*] $prefix:ident) =>
    {
/* **** **** **** **** NESTED **** **** **** **** */

stateful_macro_rules::stateful_macro_rules! {
    macro_kit_up (
        mids: ($$($$middle:ident)*),
        mname: ($$name:ident) = (must_set),
        cps_binds: ($$([ $$cps_bind:ident $$cps_fn:ident [$$($$cps_args:tt)*] ])*),
        pa_binds: ($$([ $$pa_bind:ident [$$($$pa_args:tt)*]])*),
        inside: ($$hmru:tt) = (false),
        rules: ($$(($$($$lhs:tt)*) => {$$($$rhs:tt)*};)*),
    ) when { inside: (true) } { paste::paste! {
        pub mod [< $$($$middle _)* $name >] {
            use super::*;

            #[macro_export]
            #[macro_rules_rec::recursive]
            macro_rules! [<$prefix $$($$middle _)* $name _recursive>] {
                     (@get_module_path)  => {$($module_path)*};
                     (@get_macro_prefix) => {$prefix};
                     (@get_macro_name)   => {[<$prefix $$($$middle _)* $name>]};
                $$(  ($$($$lhs)*)        => {$$($$rhs)*};  )*
            }
            pub use [<$prefix $$($$middle _)* $name _recursive>];
            pub use [<$prefix $$($$middle _)* $name _recursive>] as [<$$($$middle _)* $name _recursive>];

            #[macro_export]
            #[cps::cps]
            macro_rules! [<$prefix $$($$middle _)* $name>] {
                (@$$$$($$$$t:tt)*) => { [<$prefix $$($$middle _)* $name _recursive>]! { @$$$$($$$$t)* } };
                ($$$$($$$$t:tt)*) =>
                $$(let $$$$$$cps_bind:tt = $$cps_fn!($$($$cps_args)*) in)*
                {
                    #[skip_paste]
                    {
                        paste::paste! {
                            [<$prefix $$($$middle _)* $name _recursive>]! {
                                $$([$$$$$$cps_bind])*
                                $$([ [ $$($$pa_args)* ] ])*
                                $$$$($$$$t)*
                            }
                        }
                    }
                };
            }
            pub use [<$prefix $$($$middle _)* $name>];
            pub use [<$prefix $$($$middle _)* $name>] as [<$$($$middle _)* $name>];
        }
        use [< $$($$middle _)* $name >]::*;
    }};

    (#[cps(let $$cpsbind:ident = $$cpsfn:ident! { $$($$cpsargs:tt)* })] ...) => {
        cps_binds.append([$$cpsbind $$cpsfn [$$($$cpsargs)*]]);
    };

    (#[paste(let $$pabind:ident = $$($$paargs:tt)+)] ...) => {
        pa_binds.append([$$pabind [$$($$paargs)*]]);
    };

    (#[prefix($$mid:ident)] ...) => {
        mids.append($$mid);
    };

    (macro_rules! $nm:ident { ... }) => {
        mname.set($nm);
        inside.set(true);
    };

    ((@$$($$relhs:tt)*) => {$$($$rerhs:tt)*}; ...) when { inside: (true) } => {
        rules.append( ( @ $$($$relhs)* ) => { $$($$rerhs)* }; );
    };

    (( $$($$nmlhs:tt)*) => {$$($$nmrhs:tt)*}; ...) when { inside: (true) } => {
        rules.append( ( $$([$$$$$$cps_bind:ident])* $$([$$$$$$pa_bind:ident])* $$($$nmlhs)* ) => { $$($$nmrhs)* }; );
    };
}

/* **** **** **** **** NESTED **** **** **** **** */
    };
}

//#[cfg(test)]
mod tests {
    use super::*;

    #[cps::cps]
    macro_rules! get_module_path {
        () => {
            $crate::macros::tests
        };
    }

    make_macro_kit_up! { get_module_path }

    #[cps::cps]
    macro_rules! myfuncps {
        () => {
            some123
        };
    }

    #[apply(macro_kit_up)]
    #[cps(let test123 = myfuncps!{})]
    #[paste(let test100 = <AA $test123 _ __>)]
    #[prefix(myvar)]
    macro_rules! pat {
        (@myrec) => {
            stringify! { myrectest }
        };
        () => {
            stringify! { $test123  $test100  $self!(@get_module_path)::Wrap(()) }
        };
    }

    // cargo rustc --profile=check -- -Zunpretty=expanded
    // find this struct in expanded code, and see `TEST_CONST`
    #[allow(non_camel_case_types)]
    pub struct FIND_ME;

    const TEST_CONST: &'static str = myvar_pat!();
    const TEST_CONST2: &'static str = myvar_pat!(@myrec);
}
