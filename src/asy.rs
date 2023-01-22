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
    ($( $t:ty ),+) => {
        $(
            impl Asy for $t {
                fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
                    assert!(self.is_finite(), "value is not finite");
                    let dec = paste! { Decimal::[<from_ $t _retain>](*self) }
                        .and_then(|d| d.round_dp(opt.precision).round_sf(opt.precision));
                    if let Some(dec) = dec {
                        write!(fmt, "{}", dec.normalize())
                    } else {
                        write!(fmt, "{:.*e}", opt.precision as usize, self)
                    }
                }
            }
        )+
    };
}

impl_asy_float!(f32, f64);

macro_rules! impl_asy_display {
    ($( $t:ty ),+) => {
        $(
            impl Asy for $t {
                fn transpile(&self, w: &mut Formatter<'_>, _opt: &AsyOptions) -> FmtResult {
                    write!(w, "{}", self)
                }
            }
        )+
    };
}

impl_asy_display!(u8, u16, u32, u64, u128, usize);
impl_asy_display!(i8, i16, i32, i64, i128, isize);
impl_asy_display!(char, &str, String);

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
