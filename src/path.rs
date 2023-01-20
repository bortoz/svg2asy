use std::fmt::{Formatter, Result as FmtResult};

use usvg::{IsDefault, PaintOrder, Path, PathSegment};

use crate::asy::{transpile, transpileln, Asy};

impl Asy for PathSegment {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match &self {
            PathSegment::LineTo { x, y } => {
                transpile!(fmt, " -- ({}, {})", x, y)
            }
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                transpile!(fmt, " .. controls ({}, {})", x1, y1)?;
                transpile!(fmt, "and ({}, {})", x2, y2)?;
                transpile!(fmt, " .. ({}, {})", x, y)
            }
            PathSegment::ClosePath => transpile!(fmt, " -- cycle"),
            PathSegment::MoveTo { .. } => panic!("cannot transpile MoveTo"),
        }
    }
}

impl Asy for Path {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
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
                "\tpath p{} = ({}, {})",
                id,
                initial_point.0,
                initial_point.1
            )?;
            for segment in path {
                transpile!(fmt, "{}", segment)?;
            }
            transpileln!(fmt, ";")?;
        } else {
            transpileln!(fmt, "\tpath[] p{} = {{", id)?;
            for (initial_point, path) in paths {
                transpile!(fmt, "\t\t({}, {})", initial_point.0, initial_point.1)?;
                for segment in path {
                    segment.transpile(fmt)?;
                }
                transpileln!(fmt, ",")?;
            }
            transpileln!(fmt, "\t}};")?;
        }

        if !transform.is_default() {
            transpileln!(fmt, "\ttransform t{} = {};", id, transform)?;
        }

        macro_rules! paint {
            ($method:literal, $($pen:expr),+) => {
                if $( $pen.is_some() ) && + {
                    if transform.is_default() {
                        transpile!(fmt, "\t{}(pic, p{}", $method, id)?;
                    } else {
                        transpile!(fmt, "\t{}(pic, t{} * p{}", $method, id, id)?;
                    }
                    $( transpile!(fmt, ", {}", $pen.as_ref().unwrap())?; )+
                    transpileln!(fmt, ");")?;
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
