use std::fmt::{Formatter, Result as FmtResult};

use usvg::{Fill, FillRule, FuzzyEq, LineCap, LineJoin, Paint, Stroke};

use crate::asy::{transpile, Asy};

impl Asy for Paint {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Paint::Color(color) => color.transpile(fmt),
            Paint::LinearGradient(_) => unimplemented!("linear gradients"),
            Paint::RadialGradient(_) => unimplemented!("radial gradients"),
            Paint::Pattern(_) => unimplemented!("patterns"),
        }
    }
}

impl Asy for Stroke {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
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
        paint.transpile(fmt)?;
        if let Some(dasharray) = dasharray {
            transpile!(fmt, " + linetype(new real[]{{")?;
            for dash in dasharray {
                transpile!(fmt, "{}, ", dash)?;
            }
            transpile!(fmt, "}}")?;
            if dashoffset.fuzzy_ne(&0.) {
                transpile!(fmt, ", {}", dashoffset)?;
            }
            transpile!(fmt, ")")?;
        }
        if miterlimit.get().fuzzy_ne(&10.) {
            transpile!(fmt, " + miterlimit({})", miterlimit.get())?;
        }
        if opacity.get().fuzzy_ne(&1.) {
            transpile!(fmt, " + opacity({})", opacity.get())?;
        }
        if width.get().fuzzy_ne(&1.) {
            transpile!(fmt, " + linewidth({})", width.get())?;
        }
        match linecap {
            LineCap::Butt => transpile!(fmt, " + squarecap")?,
            LineCap::Round => {}
            LineCap::Square => transpile!(fmt, " + extendcap")?,
        };
        match linejoin {
            LineJoin::Miter => transpile!(fmt, " + miterjoin")?,
            LineJoin::Round => {}
            LineJoin::Bevel => transpile!(fmt, " + beveljoin")?,
        };
        Ok(())
    }
}

impl Asy for Fill {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let Fill {
            paint,
            opacity,
            rule,
        } = &self;
        paint.transpile(fmt)?;
        if opacity.get().fuzzy_ne(&1.) {
            transpile!(fmt, " + opacity({})", opacity.get())?;
        }
        match rule {
            FillRule::NonZero => {}
            FillRule::EvenOdd => transpile!(fmt, " + evenodd")?,
        };
        Ok(())
    }
}
