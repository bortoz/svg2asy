use std::fmt::{Formatter, Result as FmtResult};

use usvg::{FuzzyEq, Group, IsDefault, Node, NodeKind, Transform};

use crate::asy::{transpile, transpileln, Asy};

impl Asy for Transform {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        let Transform { a, b, c, d, e, f } = &self;
        transpile!(fmt, "({}, {}, {}, {}, {}, {})", e, f, a, b, c, d)
    }
}

impl Asy for (Node, &Group) {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
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
                transpile!(fmt, "{}", (child.clone(), group))?;
            }
        }

        transpileln!(fmt, "picture pic{}() {{", id)?;
        transpileln!(fmt, "\tpicture pic;")?;

        for child in self.0.children() {
            match &*child.borrow() {
                NodeKind::Group(group) => {
                    transpileln!(fmt, "\tadd(pic, pic{}());", group.id)?;
                }
                NodeKind::Path(path) => {
                    transpile!(fmt, "{}", path)?;
                }
                NodeKind::Image(_) => eprintln!("Warning: images are not supported"),
                NodeKind::Text(_) => eprintln!("Warning: text is not supported"),
            };
        }

        if transform.is_default() {
            transpileln!(fmt, "\treturn pic;")?;
            transpileln!(fmt, "}}")?;
        } else {
            transpileln!(fmt, "\ttransform t = {};", transform)?;
            transpileln!(fmt, "\treturn t * pic;")?;
            transpileln!(fmt, "}}")?;
        }
        Ok(())
    }
}
