use std::fmt::{Formatter, Result as FmtResult};

use usvg::{Fill, FillRule, FuzzyEq, LineCap, LineJoin, Paint, Stroke};

use crate::asy::{transpile, Asy};
use crate::AsyOptions;

impl Asy for Paint {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        match &self {
            Paint::Color(color) => transpile!(fmt, opt, "{}", color),
            Paint::LinearGradient(_) => unimplemented!("linear gradients"),
            Paint::RadialGradient(_) => unimplemented!("radial gradients"),
            Paint::Pattern(_) => unimplemented!("patterns"),
        }
    }
}

impl Asy for Stroke {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        let Stroke {
            paint,
            dasharray,
            dashoffset,
            miterlimit,
            opacity,
            width,
            linecap,
            linejoin,
        } = &self;

        transpile!(fmt, opt, "{}", paint)?;
        if let Some(dasharray) = dasharray {
            transpile!(fmt, opt, " + linetype(new real[]{{")?;
            for dash in dasharray {
                transpile!(fmt, opt, "{}, ", dash)?;
            }
            transpile!(fmt, opt, "}}")?;
            if dashoffset.fuzzy_ne(&0.) {
                transpile!(fmt, opt, ", {}", dashoffset)?;
            }
            transpile!(fmt, opt, ")")?;
        }
        if miterlimit.get().fuzzy_ne(&10.) {
            transpile!(fmt, opt, " + miterlimit({})", miterlimit.get())?;
        }
        if opacity.get().fuzzy_ne(&1.) {
            transpile!(fmt, opt, " + opacity({})", opacity.get())?;
        }
        if width.get().fuzzy_ne(&1.) {
            transpile!(fmt, opt, " + linewidth({})", width.get())?;
        }
        match linecap {
            LineCap::Butt => transpile!(fmt, opt, " + squarecap")?,
            LineCap::Round => {}
            LineCap::Square => transpile!(fmt, opt, " + extendcap")?,
        };
        match linejoin {
            LineJoin::Miter => transpile!(fmt, opt, " + miterjoin")?,
            LineJoin::Round => {}
            LineJoin::Bevel => transpile!(fmt, opt, " + beveljoin")?,
        };
        Ok(())
    }
}

impl Asy for Fill {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        let Fill {
            paint,
            opacity,
            rule,
        } = &self;

        transpile!(fmt, opt, "{}", paint)?;
        if opacity.get().fuzzy_ne(&1.) {
            transpile!(fmt, opt, " + opacity({})", opacity.get())?;
        }
        match rule {
            FillRule::NonZero => {}
            FillRule::EvenOdd => transpile!(fmt, opt, " + evenodd")?,
        };
        Ok(())
    }
}
