use std::fmt::{Display, Formatter, Result as FmtResult};
use std::string::String;

use paste::paste;
use rust_decimal::Decimal;

use crate::AsyOptions;

pub trait Asy {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult;
}

impl<T: Asy> Asy for &T {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        T::transpile(self, fmt, opt)
    }
}

macro_rules! impl_asy_float {
    ($t:ty) => {
        paste! {
            impl Asy for $t {
                fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
                    let dec = Decimal::[<from_ $t _retain>](*self)
                        .unwrap()
                        .round_dp(opt.precision)
                        .round_sf(opt.precision)
                        .unwrap()
                        .normalize();
                    write!(fmt, "{}", dec)
                }
            }
        }
    };
}

impl_asy_float!(f32);
impl_asy_float!(f64);

macro_rules! impl_asy_display {
    ($t:ty) => {
        impl Asy for $t {
            fn transpile(&self, w: &mut Formatter<'_>, _opt: &AsyOptions) -> FmtResult {
                write!(w, "{}", self)
            }
        }
    };
}

impl_asy_display!(u8);
impl_asy_display!(u16);
impl_asy_display!(u32);
impl_asy_display!(u64);
impl_asy_display!(usize);
impl_asy_display!(i8);
impl_asy_display!(i16);
impl_asy_display!(i32);
impl_asy_display!(i64);
impl_asy_display!(isize);
impl_asy_display!(bool);
impl_asy_display!(char);
impl_asy_display!(&str);
impl_asy_display!(String);

pub(crate) struct AsyWrapper<'a, T>(pub(crate) &'a T, pub(crate) &'a AsyOptions);

impl<T: Asy> Display for AsyWrapper<'_, T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        self.0.transpile(fmt, self.1)
    }
}

macro_rules! transpile {
    ($dst:expr, $opt:expr, $fmt:literal $( ,$arg:expr )*) => {
        write!($dst, $fmt, $( crate::asy::AsyWrapper(&$arg, $opt) ),*)
    };
}
macro_rules! transpileln {
    ($dst:expr, $opt:expr, $fmt:literal $( ,$arg:expr )*) => {
        writeln!($dst, $fmt, $( crate::asy::AsyWrapper(&$arg, $opt) ),*)
    };
}

pub(crate) use transpile;
pub(crate) use transpileln;
