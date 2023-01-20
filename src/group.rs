use std::fmt::{Formatter, Result as FmtResult};

use usvg::{FuzzyEq, Group, IsDefault, Node, NodeKind, Transform};

use crate::asy::{transpile, transpileln, Asy};
use crate::AsyOptions;

impl Asy for Transform {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        let Transform { a, b, c, d, e, f } = &self;
        transpile!(fmt, opt, "({}, {}, {}, {}, {}, {})", e, f, a, b, c, d)
    }
}

impl Asy for (Node, &Group) {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        let Group {
            id,
            transform,
            opacity,
            blend_mode,
            isolate,
            clip_path,
            mask,
            filters,
            filter_fill,
            filter_stroke,
            enable_background,
        } = &self.1;

        if opacity.get().fuzzy_ne(&1.) {
            eprintln!("Warning: group opacity is not supported");
        }
        if !blend_mode.is_default() {
            eprintln!("Warning: group blend mode is not supported");
        }
        if *isolate {
            eprintln!("Warning: group isolation is not supported");
        }
        if clip_path.is_some() {
            eprintln!("Warning: clipping is not supported");
        }
        if mask.is_some() {
            eprintln!("Warning: masks are not supported");
        }
        if !filters.is_empty() || filter_fill.is_some() || filter_stroke.is_some() {
            eprintln!("Warning: filters are not supported");
        }
        if enable_background.is_some() {
            eprintln!("Warning: enable-background is deprecated");
        }

        let mut transform = *transform;
        if self.0.parent().is_none() {
            transform.scale(1., -1.);
        }

        for child in self.0.children() {
            if let NodeKind::Group(group) = &*child.borrow() {
                transpile!(fmt, opt, "{}", (child.clone(), group))?;
            }
        }

        transpileln!(fmt, opt, "picture pic{}() {{", id)?;
        transpileln!(fmt, opt, "\tpicture pic;")?;

        for child in self.0.children() {
            match &*child.borrow() {
                NodeKind::Group(group) => {
                    transpileln!(fmt, opt, "\tadd(pic, pic{}());", group.id)?;
                }
                NodeKind::Path(path) => {
                    transpile!(fmt, opt, "{}", path)?;
                }
                NodeKind::Image(_) => eprintln!("Warning: images are not supported"),
                NodeKind::Text(_) => eprintln!("Warning: text is not supported"),
            };
        }

        if transform.is_default() {
            transpileln!(fmt, opt, "\treturn pic;")?;
            transpileln!(fmt, opt, "}}")?;
        } else {
            transpileln!(fmt, opt, "\ttransform t = {};", transform)?;
            transpileln!(fmt, opt, "\treturn t * pic;")?;
            transpileln!(fmt, opt, "}}")?;
        }
        Ok(())
    }
}
