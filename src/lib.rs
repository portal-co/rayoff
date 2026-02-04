#![no_std]
pub use core;
#[cfg(feature = "rayon")]
pub use rayon;

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a)
                .map(|$p| $b)
                .collect()
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a)
                .map(|$p| $b)
                .collect()
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::core::iter::IntoIterator::into_iter($a)
            .map(|$p| $b)
            .collect()
    };
}

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! sum {
    ($a:expr => |$p:pat_param|$b:expr) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a).map(|$p| $b).sum()
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a)
                .map(|$p| $b)
                .sum()
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! sum {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::core::iter::IntoIterator::into_iter($a).map(|$p| $b).sum()
    };
}

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! any {
    ($a:expr => |$p:pat_param|$b:expr) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a).any(|$p| $b)
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a).any(|$p| $b)
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! any {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::core::iter::IntoIterator::into_iter($a).any(|$p| $b)
    };
}

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! flat_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a)
                .flat_map(|$p| $b)
                .collect()
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a)
                .flat_map(|$p| $b)
                .collect()
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! flat_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::core::iter::IntoIterator::into_iter($a)
            .flat_map(|$p| $b)
            .collect()
    };
}

#[cfg(feature = "rayon")]
#[macro_export]
macro_rules! enumerate_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        if false {
            $crate::core::iter::IntoIterator::into_iter($a)
                .enumerate()
                .map(|$p| $b)
                .collect()
        } else {
            $crate::rayon::iter::IntoParallelIterator::into_par_iter($a)
                .enumerate()
                .map(|$p| $b)
                .collect()
        }
    };
}
#[cfg(not(feature = "rayon"))]
#[macro_export]
macro_rules! enumerate_map {
    ($a:expr => |$p:pat_param|$b:expr) => {
        $crate::core::iter::IntoIterator::into_iter($a)
            .enumerate()
            .map(|$p| $b)
            .collect()
    };
}
