//! Macros shared by the ARM NEON widening, narrowing, and shift modules.

macro_rules! unary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! binary {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! accumulate {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! multiply_accumulate {
    ($(#[$meta:meta])* $name:ident, $c:ident) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name(acc: V128, a: V128, b: V128) -> V128 {
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), b.as_ptr(), out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! shift_unary {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! shift_accumulate {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(acc: V128, a: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(acc: V128, a: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(acc.as_ptr(), a.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

macro_rules! shift_binary {
    ($(#[$meta:meta])* $name:ident, $c:ident, 0, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128, b: V128) -> V128 {
            const { assert!(N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
    ($(#[$meta:meta])* $name:ident, $c:ident, $min:expr, $max:expr) => {
        $(#[$meta])*
        #[inline(always)]
        pub unsafe fn $name<const N: u32>(a: V128, b: V128) -> V128 {
            const { assert!(N >= $min && N <= $max, "shift immediate out of range") };
            let mut out = V128::from_bytes([0; 16]);
            unsafe {
                $c(a.as_ptr(), b.as_ptr(), N as i32, out.as_mut_ptr());
            }
            out
        }
    };
}

pub(crate) use accumulate;
pub(crate) use binary;
pub(crate) use multiply_accumulate;
pub(crate) use shift_accumulate;
pub(crate) use shift_binary;
pub(crate) use shift_unary;
pub(crate) use unary;
