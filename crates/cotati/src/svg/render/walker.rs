//! A svg document visitor pattern implementation.

use std::{fs, path::Path};

use roxmltree::ParsingOptions;

use crate::{Error, Result};

/// A svg document visitor must implement this trait.
pub trait SvgWalker {}

#[allow(unused)]
struct SvgWalkerContext<'a, W> {
    walker: &'a mut W,
}

impl<'a, W> SvgWalkerContext<'a, W> {
    fn new(walker: &'a mut W) -> Self {
        Self { walker }
    }

    fn next_event(&mut self) {}
}

/// Iterate over all nodes of the svg document.
pub fn read<C: AsRef<str>, W: SvgWalker>(doc: C, walker: &mut W) -> Result<()> {
    let mut context = SvgWalkerContext::new(walker);

    let mut doc = roxmltree::Document::parse_with_options(
        doc.as_ref(),
        ParsingOptions {
            allow_dtd: true,
            ..Default::default()
        },
    )?;

    for node in doc.descendants() {
        log::trace!("{:?}", node.tag_name());
    }

    Ok(())
}

/// Iterate a svg document from file.
pub fn read_file<P: AsRef<Path>, W: SvgWalker>(path: P, walker: &mut W) -> Result<()> {
    read_file(fs::read_to_string(path)?, walker)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockWalker;

    impl SvgWalker for MockWalker {}

    #[test]
    fn group_test() {
        pretty_env_logger::init();
        read(include_str!("./testdata/group.xml"), &mut MockWalker).unwrap();
    }
}
