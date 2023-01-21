use std::fmt::{Formatter, Result as FmtResult};

use usvg::{IsDefault, PaintOrder, Path, PathSegment};

use crate::asy::{transpile, transpileln, Asy};
use crate::AsyOptions;

impl Asy for PathSegment {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        match &self {
            PathSegment::LineTo { x, y } => {
                transpile!(fmt, opt, " -- ({}, {})", x, y)
            }
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                transpile!(fmt, opt, " .. controls ({}, {})", x1, y1)?;
                transpile!(fmt, opt, " and ({}, {})", x2, y2)?;
                transpile!(fmt, opt, " .. ({}, {})", x, y)
            }
            PathSegment::ClosePath => transpile!(fmt, opt, " -- cycle"),
            PathSegment::MoveTo { .. } => panic!("cannot transpile MoveTo"),
        }
    }
}

impl Asy for Path {
    fn transpile(&self, fmt: &mut Formatter<'_>, opt: &AsyOptions) -> FmtResult {
        let Path {
            id,
            transform,
            visibility,
            fill,
            stroke,
            paint_order,
            data,
            rendering_mode: _rendering_mode,
            text_bbox: _text_bbox,
        } = &self;

        if !visibility.is_default() {
            eprintln!("Warning: path visibility is not supported");
        }

        let mut paths = Vec::new();
        let mut initial_point = (0., 0.);
        let mut last_path = Vec::new();

        for segment in data.segments() {
            match segment {
                PathSegment::MoveTo { x, y } => {
                    initial_point = (x, y);
                    paths.push((initial_point, last_path));
                    last_path = Vec::new();
                }
                PathSegment::ClosePath => {
                    last_path.push(segment);
                    paths.push((initial_point, last_path));
                    last_path = Vec::new();
                }
                _ => {
                    last_path.push(segment);
                }
            };
        }

        paths.push((initial_point, last_path));
        paths.retain(|(_, path)| !path.is_empty());

        if paths.len() == 1 {
            let (initial_point, path) = &paths[0];
            transpile!(
                fmt,
                opt,
                "\tpath p{} = ({}, {})",
                id,
                initial_point.0,
                initial_point.1
            )?;
            for segment in path {
                transpile!(fmt, opt, "{}", segment)?;
            }
            transpileln!(fmt, opt, ";")?;
        } else {
            transpileln!(fmt, opt, "\tpath[] p{} = {{", id)?;
            for (initial_point, path) in paths {
                transpile!(fmt, opt, "\t\t({}, {})", initial_point.0, initial_point.1)?;
                for segment in path {
                    transpile!(fmt, opt, "{}", segment)?;
                }
                transpileln!(fmt, opt, ",")?;
            }
            transpileln!(fmt, opt, "\t}};")?;
        }

        if !transform.is_default() {
            transpileln!(fmt, opt, "\ttransform t{} = {};", id, transform)?;
        }

        macro_rules! paint {
            ($method:literal, $($pen:expr),+) => {
                if $( $pen.is_some() ) && + {
                    if transform.is_default() {
                        transpile!(fmt, opt, "\t{}(pic, p{}", $method, id)?;
                    } else {
                        transpile!(fmt, opt, "\t{}(pic, t{} * p{}", $method, id, id)?;
                    }
                    $( transpile!(fmt, opt, ", {}", $pen.as_ref().unwrap())?; )+
                    transpileln!(fmt, opt, ");")?;
                }
            };
        }

        if fill.is_some() && stroke.is_some() && matches!(paint_order, PaintOrder::FillAndStroke) {
            paint!("filldraw", fill, stroke);
        } else {
            paint!("draw", stroke);
            paint!("fill", fill);
        };

        Ok(())
    }
}
