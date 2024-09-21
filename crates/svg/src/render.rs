use std::{fs, path::Path};

use crate::Result;

/// The svg rendering backend.
pub trait SvgRendering {}

/// Render svg file.
pub fn render_file<P: AsRef<Path>, R: SvgRendering>(svg_path: P, renderer: &mut R) -> Result<()> {
    render(fs::read_to_string(svg_path)?, renderer)
}

/// Render a svg image in memory.
pub fn render<C: AsRef<str>, R: SvgRendering>(content: C, renderer: &mut R) -> Result<()> {
    let parser = svg::read(content.as_ref())?;

    for event in parser {
        match event {
            svg::parser::Event::Error(error) => return Err(error.into()),

            svg::parser::Event::Tag(_, t, hash_map) => todo!(),
            svg::parser::Event::Text(_) => todo!(),
            svg::parser::Event::Comment(_) => todo!(),
            svg::parser::Event::Declaration(_) => todo!(),
            svg::parser::Event::Instruction(_) => todo!(),
        }
    }

    todo!()
}
