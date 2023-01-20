use std::fmt::{Formatter, Result as FmtResult};

use usvg::Color;

use crate::asy::Asy;

impl Asy for Color {
    fn transpile(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        match (self.red, self.green, self.blue) {
            (255, 192, 192) => write!(fmt, "palered"),
            (255, 128, 128) => write!(fmt, "lightred"),
            (255, 64, 64) => write!(fmt, "mediumred"),
            (255, 0, 0) => write!(fmt, "red"),
            (192, 0, 0) => write!(fmt, "heavyred"),
            (128, 0, 0) => write!(fmt, "brown"),
            (64, 0, 0) => write!(fmt, "darkbrown"),

            (192, 255, 192) => write!(fmt, "palegreen"),
            (128, 255, 128) => write!(fmt, "lightgreen"),
            (64, 255, 64) => write!(fmt, "mediumgreen"),
            (0, 255, 0) => write!(fmt, "green"),
            (0, 192, 0) => write!(fmt, "heavygreen"),
            (0, 128, 0) => write!(fmt, "deepgreen"),
            (0, 64, 0) => write!(fmt, "darkgreen"),

            (192, 192, 255) => write!(fmt, "paleblue"),
            (128, 128, 255) => write!(fmt, "lightblue"),
            (64, 64, 255) => write!(fmt, "mediumblue"),
            (0, 0, 255) => write!(fmt, "blue"),
            (0, 0, 192) => write!(fmt, "heavyblue"),
            (0, 0, 128) => write!(fmt, "deepblue"),
            (0, 0, 64) => write!(fmt, "darkblue"),

            (192, 255, 255) => write!(fmt, "palecyan"),
            (128, 255, 255) => write!(fmt, "lightcyan"),
            (64, 255, 255) => write!(fmt, "mediumcyan"),
            (0, 192, 192) => write!(fmt, "heavycyan"),
            (0, 128, 128) => write!(fmt, "deepcyan"),
            (0, 64, 64) => write!(fmt, "darkcyan"),

            (255, 192, 255) => write!(fmt, "pink"),
            (255, 128, 255) => write!(fmt, "lightmagenta"),
            (255, 64, 255) => write!(fmt, "mediummagenta"),
            (255, 0, 255) => write!(fmt, "magenta"),
            (192, 0, 192) => write!(fmt, "heavymagenta"),
            (128, 0, 128) => write!(fmt, "deepmagenta"),
            (64, 0, 64) => write!(fmt, "darkmagenta"),

            (255, 255, 192) => write!(fmt, "paleyellow"),
            (255, 255, 128) => write!(fmt, "lightyellow"),
            (255, 255, 64) => write!(fmt, "mediumyellow"),
            (255, 255, 0) => write!(fmt, "yellow"),
            (192, 192, 0) => write!(fmt, "lightolive"),
            (128, 128, 0) => write!(fmt, "olive"),
            (64, 64, 0) => write!(fmt, "darkolive"),

            (243, 243, 243) => write!(fmt, "palegray"),
            (230, 230, 230) => write!(fmt, "lightgray"),
            (192, 192, 192) => write!(fmt, "mediumgray"),
            (128, 128, 128) => write!(fmt, "gray"),
            (12, 12, 12) => write!(fmt, "darkgray"),

            (0, 0, 0) => write!(fmt, "black"),
            (255, 255, 255) => write!(fmt, "white"),
            (255, 128, 0) => write!(fmt, "orange"),
            (255, 0, 128) => write!(fmt, "fuchsia"),
            (128, 255, 0) => write!(fmt, "chartreuse"),
            (0, 255, 128) => write!(fmt, "springgreen"),
            (128, 0, 255) => write!(fmt, "purple"),
            (0, 128, 255) => write!(fmt, "royalblue"),

            (r, g, b) => write!(fmt, "rgb(\"{:02x}{:02x}{:02x}\")", r, g, b),
        }
    }
}
