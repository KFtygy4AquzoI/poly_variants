#[macro_export]
macro_rules! apply_generics {
    ([pub struct $Name:ident$(<    $( $G:ident$(: $I:ident )* ),*    >)?  $(($($tuple:tt)*))? ;]
     [$($gen:tt)*] [$($whe:tt)*]
    ) => {

      #[derive(Default, Debug, Clone, Copy)]
      pub struct $Name <  $(  $( $G $(: $I )* ,)*  )? $($gen)*   >  $(($($tuple)*))?
         where
           $($whe)*
      ;

    };

    ([impl$(<$($G:ident$(: $I:ident)?),*>)? $Trait:ident$(<$($Ty:ty),*>)? for $Type:ty { $($code:tt)* }]
     [$($gen:tt)*] [$($whe:tt)*]
    ) => {

      impl< $($($G $(: $I )?, )*)?  $($gen)*  > $Trait$(<$($Ty)*,>)? for $Type
        where
          $($whe)*
      { $($code)* }

    };

}
