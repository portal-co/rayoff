#![no_std]
pub use core;
#[cfg(feature = "rayon")]
pub use rayon;

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! frames {
    ($a:expr => $($f:ident $(:|$p:pat_param|$b:expr)?),*) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a)
                $(.$f($(|$p| $b)?))*
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a)
                $(.$f($(|$p| $b)?))*
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! frames {
    ($a:expr => $($f:ident $(:|$p:pat_param|$b:expr)?),*) => {
        $crate::core::iter::IntoIterator::into_iter($a)
            $(.$f($(|$p| $b)?))*
    };
}

#[macro_export]
macro_rules! map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::frames!($a => map:|$p| $b, collect)
    };
}

#[macro_export]
macro_rules! sum {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::frames!($a => map:|$p| $b, sum)
    };
}

#[macro_export]
macro_rules! any {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::frames!($a => any:|$p| $b)
    };
}

#[macro_export]
macro_rules! flat_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::frames!($a => flat_map:|$p| $b, collect)
    };
}

#[macro_export]
macro_rules! enumerate_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::frames!($a => enumerate, map:|$p| $b, collect)
    };
}
