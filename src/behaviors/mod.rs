use impl_trait_for_tuples::impl_for_tuples;

pub mod actor;
pub mod attack;
pub mod movement;
pub mod wait;

pub trait Behavior {
    fn behavior() -> Self;
}

#[impl_for_tuples(1, 16)]
impl Behavior for BehaviorIdentifier {
    fn behavior() -> Self {
        for_tuples!( ( #( (BehaviorIdentifier::behavior())  ),* ) )
    }
}

// If I was an adult, I'd implement this as a derive macro
#[macro_export]
macro_rules! behavior {
    ($n:ident) => {
        paste::paste! {
            pub type [< $n:camel Behavior >] = On< [<$n:camel Intent >] >;

            impl $crate::behaviors::Behavior for [< $n:camel Behavior >] {
                fn behavior() -> Self {
                    On::< [< $n:camel Intent >] >::run( [< $n >] )
                }
            }
        }
    };
}
