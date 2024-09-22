use std::{fs, path::Path};

use xml::EventReader;

use crate::Result;

/// The svg rendering backend.
pub trait SvgRendering {}

/// Render svg file.
pub fn render_file<P: AsRef<Path>, R: SvgRendering>(svg_path: P, renderer: &mut R) -> Result<()> {
    render(fs::read_to_string(svg_path)?, renderer)
}

/// Render a svg image in memory.
pub fn render<C: AsRef<str>, R: SvgRendering>(content: C, _renderer: &mut R) -> Result<()> {
    let reader = EventReader::from_str(content.as_ref());

    for event in reader {
        match event? {
            _ => {}
        }
    }
    todo!()
}
